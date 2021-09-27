use crate::regs::gpio::{Block, Port, PortConfiguration};

const OFFSET_0: u32 = 0x00;
const OFFSET_1: u32 = 0x04;
const RESET_VALUE: u32 = 0x4444_4444;

pub struct RegisterBlock {
    bits: u32,
    block: Block,
}

#[inline(always)]
pub fn read(block: Block) -> RegisterBlock {
    let address = (block.base_address() + OFFSET) as *const u32;
    RegisterBlock { bits: unsafe { *address }, block }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn write_back(&self) {
        let address = (self.block.base_address() + OFFSET) as *mut u32;
        unsafe { *address = self.bits; }
    }
}

impl RegisterBlock {
    pub fn configuration(&self, port: Port) -> Option<PortConfiguration> {
        let offset = match port {
            Port::Port0 |
            Port::Port1 |
            Port::Port2 |
            Port::Port3 |
            Port::Port4 |
            Port::Port5 |
            Port::Port6 |
            Port::Port7 => OFFSET_0,

            Port::Port8 |
            Port::Port9 |
            Port::Port10 |
            Port::Port11 |
            Port::Port12 |
            Port::Port13 |
            Port::Port14 |
            Port::Port15 => OFFSET_1
        };
    }
}