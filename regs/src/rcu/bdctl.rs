use crate::rcu::{BASE_ADDRESS, RTCClkEntrySel};

const OFFSET: u32 = 0x20;
const RESET_VALUE: u32 = 0x0000_0018;

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
    pub fn is_backup_domain_reset(&self) -> bool {
        (self.bits >> 16) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_rtc_enabled(&self) -> bool {
        (self.bits >> 15) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn rtc_clk_entry_sel(&self) -> Option<RTCClkEntrySel> {
        RTCClkEntrySel::try_from_raw_value(((self.bits >> 8) & 0b11) as u8)
    }

    #[inline(always)]
    pub fn is_lxtal_bypass_mode_enabled(&self) -> bool {
        (self.bits >> 2) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_lxtal_stable(&self) -> bool {
        (self.bits >> 1) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_lxtal_enabled(&self) -> bool {
        (self.bits >> 0) & 0x1 == 0x1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_backup_domain_reset(&mut self, value: bool) {
        const POSITION: u32 = 16;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_rtc_enabled(&mut self, value: bool) {
        const POSITION: u32 = 15;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_rtc_clk_entry_sel(&mut self, value: RTCClkEntrySel) {
        const POSITION: u32 = 8;
        const MASK: u32 = 0b11;
        self.bits = (self.bits & !(MASK << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn set_lxtal_bypass_mode_enabled(&mut self, value: bool) {
        const POSITION: u32 = 2;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_lxtal_enabled(&mut self, value: bool) {
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