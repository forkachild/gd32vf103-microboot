use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    static __sheap: usize;
}

const PAGE_SIZE: u32 = 8;
const MAX_SIZE: u32 = 8 * 1024;

struct UAlloc;

unsafe impl GlobalAlloc for UAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        0 as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {

    }
}

#[global_allocator]
static GLOBAL_ALLOC: UAlloc = UAlloc;