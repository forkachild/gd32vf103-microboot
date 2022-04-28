use crate::rcu::BASE_ADDRESS;

const OFFSET: u32 = 0x1C;
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
    pub fn is_dac_enabled(&self) -> bool {
        (self.bits >> 29) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_power_control_enabled(&self) -> bool {
        (self.bits >> 28) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_backup_interface_enabled(&self) -> bool {
        (self.bits >> 27) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_can1_enabled(&self) -> bool {
        (self.bits >> 26) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_can0_enabled(&self) -> bool {
        (self.bits >> 25) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_i2c1_enabled(&self) -> bool {
        (self.bits >> 22) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_i2c0_enabled(&self) -> bool {
        (self.bits >> 21) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_uart4_enabled(&self) -> bool {
        (self.bits >> 20) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_uart3_enabled(&self) -> bool {
        (self.bits >> 19) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_uart2_enabled(&self) -> bool {
        (self.bits >> 18) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_uart1_enabled(&self) -> bool {
        (self.bits >> 17) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_spi2_enabled(&self) -> bool {
        (self.bits >> 15) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_spi1_enabled(&self) -> bool {
        (self.bits >> 14) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_wwdgt_enabled(&self) -> bool {
        (self.bits >> 11) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_timer6_enabled(&self) -> bool {
        (self.bits >> 5) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_timer5_enabled(&self) -> bool {
        (self.bits >> 4) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_timer4_enabled(&self) -> bool {
        (self.bits >> 3) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_timer3_enabled(&self) -> bool {
        (self.bits >> 2) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_timer2_enabled(&self) -> bool {
        (self.bits >> 1) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_timer1_enabled(&self) -> bool {
        (self.bits & 0x1 >> 0) == 0x1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_dac_enabled(&mut self, value: bool) {
        const POSITION: u32 = 29;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_power_control_enabled(&mut self, value: bool) {
        const POSITION: u32 = 28;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_backup_interface_enabled(&mut self, value: bool) {
        const POSITION: u32 = 27;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_can1_enabled(&mut self, value: bool) {
        const POSITION: u32 = 26;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_can0_enabled(&mut self, value: bool) {
        const POSITION: u32 = 25;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_i2c1_enabled(&mut self, value: bool) {
        const POSITION: u32 = 22;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_i2c0_enabled(&mut self, value: bool) {
        const POSITION: u32 = 21;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_uart4_enabled(&mut self, value: bool) {
        const POSITION: u32 = 20;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_uart3_enabled(&mut self, value: bool) {
        const POSITION: u32 = 19;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_uart2_enabled(&mut self, value: bool) {
        const POSITION: u32 = 18;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_uart1_enabled(&mut self, value: bool) {
        const POSITION: u32 = 17;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_spi2_enabled(&mut self, value: bool) {
        const POSITION: u32 = 15;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_spi1_enabled(&mut self, value: bool) {
        const POSITION: u32 = 14;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_wwdgt_enabled(&mut self, value: bool) {
        const POSITION: u32 = 11;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_timer6_enabled(&mut self, value: bool) {
        const POSITION: u32 = 5;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_timer5_enabled(&mut self, value: bool) {
        const POSITION: u32 = 4;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_timer4_enabled(&mut self, value: bool) {
        const POSITION: u32 = 3;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_timer3_enabled(&mut self, value: bool) {
        const POSITION: u32 = 2;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_timer2_enabled(&mut self, value: bool) {
        const POSITION: u32 = 1;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn set_timer1_enabled(&mut self, value: bool) {
        const POSITION: u32 = 0;
        if value {
            self.bits |= 1 << POSITION;
        } else {
            self.bits &= !1 << POSITION;
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.bits = RESET_VALUE;
    }
}