use crate::rcu::BASE_ADDRESS;

const OFFSET: u32 = 0x00;
const RESET_VALUE: u32 = 0x0000_0083;

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
    pub fn is_pll2_clk_stable(&self) -> bool {
        (self.bits >> 29) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_pll2_enabled(&self) -> bool {
        (self.bits >> 28) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_pll1_clk_stable(&self) -> bool {
        (self.bits >> 27) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_pll1_enabled(&self) -> bool {
        (self.bits >> 26) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_pll_clk_stable(&self) -> bool {
        (self.bits >> 25) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_pll_enabled(&self) -> bool {
        (self.bits >> 24) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_hxtal_clk_monitor_enabled(&self) -> bool {
        (self.bits >> 19) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_hxtal_clk_bypass_mode_enabled(&self) -> bool {
        (self.bits >> 18) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_hxtal_clk_stable(&self) -> bool {
        (self.bits >> 17) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_hxtal_clk_enabled(&self) -> bool {
        (self.bits >> 16) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn irc_osc_calib_value(&self) -> u8 {
        ((self.bits >> 8) & 0xFF) as u8
    }

    #[inline(always)]
    pub fn irc_osc_clk_trim_adjust_value(&self) -> u8 {
        ((self.bits >> 3) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn is_irc_osc_clk_stable(&self) -> bool {
        (self.bits >> 1) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_irc_osc_clk_enabled(&self) -> bool {
        self.bits & 0x1 == 0x1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_pll2_clk_enabled(&mut self, value: bool) {
        const POSITION: u32 = 28;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_pll1_clk_enabled(&mut self, value: bool) {
        const POSITION: u32 = 26;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_pll_clk_enabled(&mut self, value: bool) {
        const POSITION: u32 = 24;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_hxtal_clk_monitor_enabled(&mut self, value: bool) {
        const POSITION: u32 = 19;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_hxtal_clk_bypass_mode_enabled(&mut self, value: bool) {
        const POSITION: u32 = 18;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_hxtal_clk_enabled(&mut self, value: bool) {
        const POSITION: u32 = 16;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_irc_osc_clk_trim_adjust_value(&mut self, value: u8) {
        self.bits = (self.bits & !(0x1F << 3)) | ((value as u32) << 3);
    }

    #[inline(always)]
    pub fn set_irc_osc_clk_enabled(&mut self, value: bool) {
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