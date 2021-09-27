use crate::regs::rcu::BASE_ADDRESS;

const OFFSET: u32 = 0x08;
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
    pub fn pll2_stabilized_int_enabled(&self) -> bool {
        (self.bits >> 14) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn pll1_stabilized_int_enabled(&self) -> bool {
        (self.bits >> 13) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn pll_stabilized_int_enabled(&self) -> bool {
        (self.bits >> 12) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn hxtal_stabilized_int_enabled(&self) -> bool {
        (self.bits >> 11) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn irc_8m_stabilized_int_enabled(&self) -> bool {
        (self.bits >> 10) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn lxtal_stabilized_int_enabled(&self) -> bool {
        (self.bits >> 9) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn irc_40k_stabilized_int_enabled(&self) -> bool {
        (self.bits >> 8) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn hxtal_clk_stuck_int_flag(&self) -> bool {
        (self.bits >> 7) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn pll2_stabilized_int_flag(&self) -> bool {
        (self.bits >> 6) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn pll1_stabilized_int_flag(&self) -> bool {
        (self.bits >> 5) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn pll_stabilized_int_flag(&self) -> bool {
        (self.bits >> 4) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn hxtal_stabilized_int_flag(&self) -> bool {
        (self.bits >> 3) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn irc_8m_stabilized_int_flag(&self) -> bool {
        (self.bits >> 2) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn lxtal_stabilized_int_flag(&self) -> bool {
        (self.bits >> 1) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn irc_40k_stabilized_int_flag(&self) -> bool {
        self.bits & 0x1 == 0x1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn clear_hxtal_clk_stuck_int(&mut self) {
        self.bits |= (1 << 23);
    }

    #[inline(always)]
    pub fn clear_pll2_stabilized_int(&mut self) {
        self.bits |= (1 << 22);
    }

    #[inline(always)]
    pub fn clear_pll1_stabilized_int(&mut self) {
        self.bits |= (1 << 21);
    }

    #[inline(always)]
    pub fn clear_pll_stabilized_int(&mut self) {
        self.bits |= (1 << 20);
    }

    #[inline(always)]
    pub fn clear_hxtal_stabilized_int(&mut self) {
        self.bits |= (1 << 19);
    }

    #[inline(always)]
    pub fn clear_irc_8m_stabilized_int(&mut self) {
        self.bits |= (1 << 18);
    }

    #[inline(always)]
    pub fn clear_lxtal_stabilized_int(&mut self) {
        self.bits |= (1 << 17);
    }

    #[inline(always)]
    pub fn clear_irc_40k_stabilized_int(&mut self) {
        self.bits |= (1 << 16);
    }

    #[inline(always)]
    pub fn set_pll2_stabilized_int_enabled(&mut self, value: bool) {
        const POSITION: u32 = 14;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_pll1_stabilized_int_enabled(&mut self, value: bool) {
        const POSITION: u32 = 13;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_pll_stabilized_int_enabled(&mut self, value: bool) {
        const POSITION: u32 = 12;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_hxtal_stabilized_int_enabled(&mut self, value: bool) {
        const POSITION: u32 = 11;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_irc_8m_stabilized_int_enabled(&mut self, value: bool) {
        const POSITION: u32 = 10;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_lxtal_stabilized_int_enabled(&mut self, value: bool) {
        const POSITION: u32 = 9;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_irc_40k_stabilized_int_enabled(&mut self, value: bool) {
        const POSITION: u32 = 8;
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