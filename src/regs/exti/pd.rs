use crate::regs::exti::{BASE_ADDRESS, EXTILine};

const OFFSET: u32 = 0x14;

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
    pub fn is_interrupt_pending(&self, exti_line: EXTILine) -> bool {
        (self.bits >> exti_line.raw_value()) & 0b1 == 0b1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn clear_pending_interrupt(&mut self, exti_line: EXTILine) {
        let position = exti_line.raw_value();
        self.bits |= (1 << position);
    }
}