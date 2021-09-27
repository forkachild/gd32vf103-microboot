use crate::regs::exti::{BASE_ADDRESS, EXTILine};

const OFFSET: u32 = 0x04;
const RESET_VALUE: u32 = 0x0000_0000;

#[repr(transparent)]
pub struct RegisterBlock {
    bits: u32,
}

#[inline(always)]
pub fn read() -> RegisterBlock {
    let address = (BASE_ADDRESS + OFFSET) as *const u32;
    RegisterBlock { bits: unsafe { *address } }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn write_back(&self) {
        let address = (BASE_ADDRESS + OFFSET) as *mut u32;
        unsafe { *address = self.bits; }
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn is_event_enabled(&self, exti_line: EXTILine) -> bool {
        (self.bits >> exti_line.raw_value()) & 0b1 == 0b1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_event_enabled(&mut self, exti_line: EXTILine, value: bool) {
        let position = exti_line.raw_value();
        if value {
            self.bits |= (1 << position);
        } else {
            self.bits &= !(1 << position);
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.bits = RESET_VALUE;
    }
}