use crate::regs::rcu::BASE_ADDRESS;

const OFFSET: u32 = 0x24;
const RESET_VALUE: u32 = 0x0C00_0000;

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
    pub fn is_low_power_reset(&self) -> bool {
        (self.bits >> 31) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_wwdgt_reset(&self) -> bool {
        (self.bits >> 30) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_fwdgt_reset(&self) -> bool {
        (self.bits >> 29) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_software_reset(&self) -> bool {
        (self.bits >> 28) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_power_reset(&self) -> bool {
        (self.bits >> 27) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_external_pin_reset(&self) -> bool {
        (self.bits >> 26) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_reset_flag_clear(&self) -> bool {
        (self.bits >> 24) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_irc_40k_stable(&self) -> bool {
        (self.bits >> 1) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_irc_40k_enabled(&self) -> bool {
        (self.bits >> 0) & 0x1 == 0x1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_reset_flag_clear(&mut self, value: bool) {
        const POSITION: u32 = 24;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_irc_40k_enabled(&mut self, value: bool) {
        const POSITION: u32 = 0;
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