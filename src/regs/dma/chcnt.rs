use crate::regs::dma::{Bank, Channel};

const OFFSET: u32 = 0x0C;
const CHANNEL_BANK_SIZE: u32 = 0x14;

pub struct RegisterBlock {
    bits: u32,
    bank: Bank,
    channel: Channel,
}

#[inline]
pub fn read(bank: Bank, channel: Channel) -> RegisterBlock {
    let address =
        (bank.base_address() + OFFSET + (CHANNEL_BANK_SIZE * channel.number() as u32)) as *const u32;
    RegisterBlock { bits: unsafe { *address }, bank, channel }
}

impl RegisterBlock {
    #[inline]
    pub fn write_back(&self) {
        let address =
            (self.bank.base_address() + OFFSET + (CHANNEL_BANK_SIZE * self.channel.number() as u32)) as *mut u32;
        unsafe { *address = self.bits; }
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn count(&self) -> u32 {
        self.bits & 0xFFFF
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_count(&mut self, value: u32) {
        self.bits = value & 0xFFFF;
    }
}