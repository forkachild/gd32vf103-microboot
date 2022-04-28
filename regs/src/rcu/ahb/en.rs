use crate::rcu::BASE_ADDRESS;

const OFFSET: u32 = 0x14;
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
    pub fn is_usbfs_clk_enabled(&self) -> bool {
        (self.bits >> 12) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_exmc_clk_enabled(&self) -> bool {
        (self.bits >> 8) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_crc_clk_enabled(&self) -> bool {
        (self.bits >> 6) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_fmc_clk_enabled_sleep_mode(&self) -> bool {
        (self.bits >> 4) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_sram_iface_clk_enabled_sleep_mode(&self) -> bool {
        (self.bits >> 2) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_dma1_clk_enabled(&self) -> bool {
        (self.bits >> 1) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_dma0_clk_enabled(&self) -> bool {
        (self.bits >> 0) & 0x1 == 0x1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_usbfs_clk_enabled(&mut self, value: bool) {
        const POSITION: u32 = 12;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_exmc_clk_enabled(&mut self, value: bool) {
        const POSITION: u32 = 8;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_crc_clk_enabled(&mut self, value: bool) {
        const POSITION: u32 = 6;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_fmc_clk_enabled_sleep_mode(&mut self, value: bool) {
        const POSITION: u32 = 4;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_sram_iface_clk_enabled_sleep_mode(&mut self, value: bool) {
        const POSITION: u32 = 2;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_dma1_clk_enabled(&mut self, value: bool) {
        const POSITION: u32 = 1;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_dma0_clk_enabled(&mut self, value: bool) {
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