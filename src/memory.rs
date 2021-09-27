use core::ops::{Deref, DerefMut};

#[repr(transparent)]
pub struct ROMem<T>(pub *const T);

#[repr(transparent)]
pub struct RWMem<T>(pub *mut T);

impl<T> Deref for ROMem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> Deref for RWMem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> DerefMut for RWMem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

gen_ro_mem!(u8);
gen_ro_mem!(u16);
gen_ro_mem!(u32);
gen_ro_mem!(usize);
gen_rw_mem!(u8);
gen_rw_mem!(u16);
gen_rw_mem!(u32);
gen_rw_mem!(usize);
