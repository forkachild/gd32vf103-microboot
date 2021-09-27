use crate::regs::dma::{Bank, Channel};

#[repr(transparent)]
pub struct RegisterBlock {
    bits: u32,
}

#[inline(always)]
pub fn read(bank: Bank) -> RegisterBlock {
    let address = (bank.base_address() + 0x00) as *const u32;
    RegisterBlock { bits: unsafe { *address } }
}

impl RegisterBlock {
    /// Returns the error flag of the channel
    #[inline(always)]
    pub fn is_transfer_error(&self, channel: Channel) -> bool {
        self.get(channel, 3)
    }

    /// Returns the half transfer completed flag of the channel
    #[inline(always)]
    pub fn is_half_transfer_finished(&self, channel: Channel) -> bool {
        self.get(channel, 2)
    }

    /// Returns the full transfer completed flag of the channel
    #[inline(always)]
    pub fn is_full_transfer_finished(&self, channel: Channel) -> bool {
        self.get(channel, 1)
    }

    /// Returns the global interrupt flag of the channel
    #[inline(always)]
    pub fn is_global_interrupt_flag_set(&self, channel: Channel) -> bool {
        self.get(channel, 0)
    }

    #[inline(always)]
    fn get(&self, channel: Channel, offset: usize) -> bool {
        let shift_by = (4 * channel.number() as usize) + offset;
        (self.bits >> shift_by) & 0x1 == 0x1
    }
}