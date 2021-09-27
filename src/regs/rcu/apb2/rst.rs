use crate::regs::rcu::BASE_ADDRESS;

const OFFSET: u32 = 0x0C;
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
    pub fn is_usart0_reset(&self) -> bool {
        (self.bits >> 14) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_spi0_reset(&self) -> bool {
        (self.bits >> 12) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_timer0_reset(&self) -> bool {
        (self.bits >> 11) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_adc1_reset(&self) -> bool {
        (self.bits >> 10) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_adc0_reset(&self) -> bool {
        (self.bits >> 9) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_gpio_e_reset(&self) -> bool {
        (self.bits >> 6) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_gpio_d_reset(&self) -> bool {
        (self.bits >> 5) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_gpio_c_reset(&self) -> bool {
        (self.bits >> 4) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_gpio_b_reset(&self) -> bool {
        (self.bits >> 3) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_gpio_a_reset(&self) -> bool {
        (self.bits >> 2) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_afio_reset(&self) -> bool {
        (self.bits >> 0) & 0x1 == 0x1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_usart0_reset(&mut self, value: bool) {
        const POSITION: u32 = 14;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_spi0_reset(&mut self, value: bool) {
        const POSITION: u32 = 12;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_timer0_reset(&mut self, value: bool) {
        const POSITION: u32 = 11;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_adc1_reset(&mut self, value: bool) {
        const POSITION: u32 = 10;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_adc0_reset(&mut self, value: bool) {
        const POSITION: u32 = 9;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_gpio_e_reset(&mut self, value: bool) {
        const POSITION: u32 = 6;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_gpio_d_reset(&mut self, value: bool) {
        const POSITION: u32 = 5;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_gpio_c_reset(&mut self, value: bool) {
        const POSITION: u32 = 4;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_gpio_b_reset(&mut self, value: bool) {
        const POSITION: u32 = 3;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_gpio_a_reset(&mut self, value: bool) {
        const POSITION: u32 = 2;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_afio_reset(&mut self, value: bool) {
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