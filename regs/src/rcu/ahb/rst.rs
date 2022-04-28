use crate::rcu::BASE_ADDRESS;

const OFFSET: u32 = 0x28;
const RESET_VALUE: u32 = 0x0000_0000;

#[repr(transparent)]
pub struct RegisterBlock {
    bits: u32,
}

#[inline]
pub fn read() -> RegisterBlock {
    let address = (BASE_ADDRESS + OFFSET) as *const u32;
    RegisterBlock { bits: unsafe { *address } }
}

impl RegisterBlock {
    #[inline]
    pub fn write_back(&self) {
        let address = (BASE_ADDRESS + OFFSET) as *mut u32;
        unsafe { *address = self.bits; }
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn is_usbfs_clk_reset(&self) -> bool {
        (self.bits >> 12) & 0x1 == 0x1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_usbfs_clk_reset(&mut self, value: bool) {
        const POSITION: u32 = 12;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.bits = RESET_VALUE;
    }
}