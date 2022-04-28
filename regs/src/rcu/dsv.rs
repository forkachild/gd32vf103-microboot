use crate::rcu::{BASE_ADDRESS, CoreVoltageSel};

const OFFSET: u32 = 0x34;
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
    pub fn deep_sleep_core_voltage_sel(&self) -> Option<CoreVoltageSel> {
        CoreVoltageSel::try_from_raw_value((self.bits & 0b11) as u8)
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_deep_sleep_core_voltage_sel(&mut self, value: CoreVoltageSel) {
        const POSITION: u32 = 0;
        const MASK: u32 = 0b11;
        self.bits = (self.bits & !(MASK << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.bits = RESET_VALUE;
    }
}