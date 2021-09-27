macro_rules! gen_ro_mem {
    ($type:ty) => {
        impl $crate::memory::ROMem<$type> {
            #[inline(always)]
            pub fn get(&self) -> $type {
                unsafe { *self.0 }
            }
        }
    }
}

macro_rules! gen_rw_mem {
    ($type:ty) => {
        impl $crate::memory::RWMem<$type> {
            #[inline(always)]
            pub fn get(&self) -> $type {
                unsafe { *self.0 }
            }

            #[inline(always)]
            pub fn set(&mut self, value: $type) {
                unsafe { *self.0 = value }
            }
        }
    }
}

macro_rules! bit_mask {
    ($($value:literal),+) => (0 as usize $(| (1 << $value))+);
    ($type:ty;$($value:literal),+) => (0 as $type $(| (1 << $value))+);
}

macro_rules! bit_mask_inv {
    ($($value:literal),+) => (!(0 as usize $(| (1 << $value))+));
    ($type:ty;$($value:literal),+) => (!(0 as $type $(| (1 << $value))+));
}