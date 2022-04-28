use crate::dma::{Bank, Channel};

const OFFSET: usize = 0x14;
const CHANNEL_BANK_SIZE: usize = 0x14;

pub struct RegisterBlock {
    bits: u32,
    bank: Bank,
    channel: Channel,
}

#[inline]
pub fn read(bank: Bank, channel: Channel) -> RegisterBlock {
    let address = (bank.base_address() + OFFSET + channel.offset()) as *const u32;
    RegisterBlock {
        bits: unsafe { *address },
        bank,
        channel,
    }
}

impl RegisterBlock {
    #[inline]
    pub fn write_back(&self) {
        let address = (self.bank.base_address() + OFFSET + self.channel.offset()) as *mut u32;
        unsafe {
            *address = self.bits;
        }
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn address(&self) -> u32 {
        self.bits
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_address(&mut self, value: u32) {
        self.bits = value;
    }
}
