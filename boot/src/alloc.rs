use core::alloc::{GlobalAlloc, Layout};
use core::mem::size_of;
use core::ops::{Deref, DerefMut};
use core::ptr::null_mut;

use crate::startup::{abort, end_heap, start_heap};

pub unsafe fn alloc_init() {
    GLOBAL_ALLOCATOR.init();
}

type AllocBlockDescPtr = *mut AllocBlockDesc;

#[derive(Debug, Copy, Clone)]
struct AllocHeap;

impl AllocHeap {
    unsafe fn init(&self) {
        self.as_block().write();
    }

    #[inline]
    fn start_ptr(&self) -> AllocBlockDescPtr {
        unsafe { &start_heap as *const usize as _ }
    }

    #[inline]
    fn end_ptr(&self) -> AllocBlockDescPtr {
        unsafe { &end_heap as *const usize as _ }
    }

    fn size(&self) -> usize {
        self.end_ptr() as usize - self.start_ptr() as usize
    }

    fn as_block(&self) -> AllocBlock {
        AllocBlock::from_parts(self.start_ptr(), self.size(), false)
    }

    fn first_block(&self) -> AllocBlock {
        unsafe { AllocBlock::from_top_block_desc_ptr_unchecked(self.start_ptr()) }
    }

    fn block_iter(&self) -> AllocBlockIter {
        AllocBlockIter::new(self.first_block(), self.clone())
    }

    fn aligned_size(layout: Layout) -> usize {
        #[cfg(target_pointer_width = "32")]
        const MINIMUM_ALIGNMENT: usize = 4;
        #[cfg(target_pointer_width = "64")]
        const MINIMUM_ALIGNMENT: usize = 8;

        let size = layout.size();
        let align = layout.align();
        let align = if align < MINIMUM_ALIGNMENT {
            MINIMUM_ALIGNMENT
        } else {
            align
        };

        (size + align - 1) & !(align - 1)
    }
}

#[alloc_error_handler]
pub fn alloc_error_handler(_: Layout) -> ! {
    unsafe { abort() };
}

#[global_allocator]
static GLOBAL_ALLOCATOR: AllocHeap = AllocHeap;

unsafe impl Sync for AllocHeap {}

unsafe impl GlobalAlloc for AllocHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = Self::aligned_size(layout);

        FirstFitAllocBlockFinder::find(size, self)
            .map_or(null_mut(), |mut block| {
                block.allocate_unchecked(size);
                block.mem_ptr()
            })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if let Some(mut block) = AllocBlock::from_mem_ptr(ptr, self) {
            block.free(self);
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct AllocBlockIter {
    block: Option<AllocBlock>,
    heap: AllocHeap,
}

impl AllocBlockIter {
    const fn new(block: AllocBlock, heap: AllocHeap) -> Self {
        Self { block: Some(block), heap }
    }
}

impl Iterator for AllocBlockIter {
    type Item = AllocBlock;

    fn next(&mut self) -> Option<Self::Item> {
        let current_block = self.block.clone();
        self.block = self.block.and_then(|block| block.next_block(&self.heap));
        current_block
    }
}

trait AllocBlockFinder {
    fn find(size: usize, heap: &AllocHeap) -> Option<AllocBlock>;
}

#[derive(Debug, Copy, Clone)]
struct BestFitAllocBlockFinder;

impl AllocBlockFinder for BestFitAllocBlockFinder {
    fn find(size: usize, heap: &AllocHeap) -> Option<AllocBlock> {
        heap
            .block_iter()
            .filter(|block| block.is_free() && block.size() >= size)
            .min_by_key(|block| block.size())
    }
}

#[derive(Debug, Copy, Clone)]
struct FirstFitAllocBlockFinder;

impl AllocBlockFinder for FirstFitAllocBlockFinder {
    fn find(size: usize, heap: &AllocHeap) -> Option<AllocBlock> {
        heap
            .block_iter()
            .find(|block| block.is_free() && block.size() >= size)
    }
}

#[derive(Debug, Copy, Clone)]
struct AllocBlock {
    top_block_desc_ptr: AllocBlockDescPtr,
    block_desc: AllocBlockDesc,
}

impl AllocBlock {
    const fn new(top_block_desc_ptr: AllocBlockDescPtr, block_desc: AllocBlockDesc) -> Self {
        Self { top_block_desc_ptr, block_desc }
    }

    const fn from_parts(top_block_desc_ptr: AllocBlockDescPtr, block_size: usize, allocated: bool) -> Self {
        let block_desc = AllocBlockDesc::from_block_size(block_size, allocated);
        Self::new(top_block_desc_ptr, block_desc)
    }

    unsafe fn from_top_block_desc_ptr_unchecked(top_block_desc_ptr: AllocBlockDescPtr) -> Self {
        let block_desc = top_block_desc_ptr.read();
        Self::new(top_block_desc_ptr, block_desc)
    }

    fn from_top_block_desc_ptr(
        top_block_desc_ptr: AllocBlockDescPtr,
        heap: &AllocHeap
    ) -> Option<Self> {
        if top_block_desc_ptr >= heap.start_ptr() {
            let block = unsafe { Self::from_top_block_desc_ptr_unchecked(top_block_desc_ptr) };

            if block.bottom_desc_ptr() as usize <=
                heap.end_ptr() as usize - AllocBlockDesc::footprint() {
                Some(block)
            } else {
                None
            }
        } else {
            None
        }
    }

    unsafe fn mem_ptr(&self) -> *mut u8 {
        (self.top_block_desc_ptr as usize + AllocBlockDesc::footprint()) as _
    }

    fn from_mem_ptr(ptr: *mut u8, heap: &AllocHeap) -> Option<Self> {
        let desc_ptr = (ptr as usize - AllocBlockDesc::footprint()) as AllocBlockDescPtr;
        Self::from_top_block_desc_ptr(desc_ptr, heap)
    }

    fn prev_block(&self, heap: &AllocHeap) -> Option<AllocBlock> {
        if self.top_block_desc_ptr > heap.start_ptr() {
            let prev_block_bottom_desc_ptr = self.prev_block_bottom_desc_ptr();
            let prev_block_desc = unsafe { prev_block_bottom_desc_ptr.read() };
            let prev_desc_ptr = (
                prev_block_bottom_desc_ptr as usize
                    - prev_block_desc.size()
                    - AllocBlockDesc::footprint()
            ) as AllocBlockDescPtr;

            Some(Self::new(prev_desc_ptr, prev_block_desc))
        } else {
            None
        }
    }

    fn next_block(&self, heap: &AllocHeap) -> Option<AllocBlock> {
        let next_block_desc_ptr = self.next_block_top_desc_ptr();
        if next_block_desc_ptr < heap.end_ptr() {
            let next_block_desc = unsafe { next_block_desc_ptr.read() };

            Some(Self::new(next_block_desc_ptr, next_block_desc))
        } else {
            None
        }
    }

    fn top_desc_ptr(&self) -> AllocBlockDescPtr {
        self.top_block_desc_ptr
    }

    fn bottom_desc_ptr(&self) -> AllocBlockDescPtr {
        (self.top_block_desc_ptr as usize
            + self.block_desc.size()
            + AllocBlockDesc::footprint()
        ) as AllocBlockDescPtr
    }

    unsafe fn allocate_unchecked(&mut self, size: usize) {
        if self.size() - size > AllocBlockDesc::pair_footprint() {
            self.allocate_split_unchecked(size);
        } else {
            self.allocate_in_place_unchecked();
        }
    }

    unsafe fn allocate_in_place_unchecked(&mut self) {
        self.block_desc.set_allocated(true);
        self.write();
    }

    unsafe fn allocate_split_unchecked(&mut self, size: usize) {
        let remaining_size = self.block_desc.size() - size - AllocBlockDesc::pair_footprint();
        self.block_desc = AllocBlockDesc::new(size, true);
        self.write();

        let next_desc_ptr = self.next_block_top_desc_ptr();
        let next_desc = AllocBlockDesc::new(remaining_size, false);
        let next_block = Self::new(next_desc_ptr, next_desc);
        next_block.write();
    }

    unsafe fn free(&mut self, heap: &AllocHeap) {
        if self.block_desc.is_free() {
            return;
        }

        let mut block_size = self.block_desc.size();
        let mut block_desc_ptr = self.top_block_desc_ptr;

        if let Some(prev_block) = self.prev_block(heap) {
            if prev_block.block_desc.is_free() {
                block_size += AllocBlockDesc::pair_footprint() + prev_block.block_desc.size();
                block_desc_ptr = prev_block.top_block_desc_ptr;
            }
        }

        if let Some(next_block) = self.next_block(heap) {
            if next_block.block_desc.is_free() {
                block_size += AllocBlockDesc::pair_footprint() + next_block.block_desc.size();
            }
        }

        self.top_block_desc_ptr = block_desc_ptr;
        self.block_desc = AllocBlockDesc::new(block_size, false);
        self.write();
    }

    fn prev_block_bottom_desc_ptr(&self) -> AllocBlockDescPtr {
        (self.top_block_desc_ptr as usize - AllocBlockDesc::footprint()) as AllocBlockDescPtr
    }

    fn next_block_top_desc_ptr(&self) -> AllocBlockDescPtr {
        (self.top_block_desc_ptr as usize
            + self.block_desc.size()
            + AllocBlockDesc::pair_footprint()
        ) as AllocBlockDescPtr
    }

    unsafe fn write(&self) {
        self.top_block_desc_ptr.write(self.block_desc);
        self.bottom_desc_ptr().write(self.block_desc);
    }
}

impl Deref for AllocBlock {
    type Target = AllocBlockDesc;

    fn deref(&self) -> &Self::Target {
        &self.block_desc
    }
}

impl DerefMut for AllocBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.block_desc
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
struct AllocBlockDesc(usize);

impl AllocBlockDesc {
    const fn new(size: usize, allocated: bool) -> Self {
        let value = size & !0x3;

        let value = if allocated {
            value | 0x1
        } else {
            value
        };

        Self(value)
    }

    const fn from_block_size(block_size: usize, allocated: bool) -> Self {
        let size = block_size - Self::pair_footprint();
        Self::new(size, allocated)
    }

    fn size(&self) -> usize {
        self.0 & !0x3
    }

    fn set_size(&mut self, size: usize) {
        self.0 = (self.0 & 0x3) | (size & !0x3);
    }

    fn is_allocated(&self) -> bool {
        self.0 & 0x1 != 0
    }

    fn is_free(&self) -> bool {
        !self.is_allocated()
    }

    fn set_allocated(&mut self, allocated: bool) {
        self.0 = if allocated {
            self.0 | 0x1
        } else {
            self.0 & !0x1
        };
    }

    const fn footprint() -> usize {
        size_of::<Self>()
    }

    const fn pair_footprint() -> usize {
        2 * Self::footprint()
    }
}
