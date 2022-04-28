use crate::dma::{Bank, Channel};

const OFFSET: u32 = 0x10;
const CHANNEL_BANK_SIZE: u32 = 0x14;

pub struct ChPAddr {
    bits: u32,
    bank: Bank,
    channel: Channel,
}

impl ChPAddr {
    #[inline(always)]
    pub fn address(&self) -> u32 {
        self.bits
    }
}

impl ChPAddr {
    #[inline(always)]
    pub fn set_address(&mut self, value: u32) {
        self.bits = value;
    }
}
