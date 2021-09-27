use crate::regs::dma::{AddressGenMode, Bank, Channel, PriorityLevel, TransferDirection, TransferSize};

const OFFSET: u32 = 0x08;
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
    pub fn is_mem_to_mem(&self) -> bool {
        (self.bits >> 14) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn priority_level(&self) -> Option<PriorityLevel> {
        PriorityLevel::from_value(((self.bits >> 12) & 0x2) as u8)
    }

    #[inline(always)]
    pub fn mem_transfer_size(&self) -> Option<TransferSize> {
        TransferSize::from_value(((self.bits >> 10) & 0x2) as u8)
    }

    #[inline(always)]
    pub fn peri_transfer_size(&self) -> Option<TransferSize> {
        TransferSize::from_value(((self.bits >> 8) & 0x2) as u8)
    }

    #[inline(always)]
    pub fn mem_next_address_gen_mode(&self) -> Option<AddressGenMode> {
        AddressGenMode::from_value(((self.bits >> 7) & 0x1) as u8)
    }

    #[inline(always)]
    pub fn peri_next_address_gen_mode(&self) -> Option<AddressGenMode> {
        AddressGenMode::from_value(((self.bits >> 6) & 0x1) as u8)
    }

    #[inline(always)]
    pub fn is_circular_mode_enabled(&self) -> bool {
        (self.bits >> 5) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn transfer_direction(&self) -> Option<TransferDirection> {
        TransferDirection::from_value(((self.bits >> 4) & 0x1) as u8)
    }

    #[inline(always)]
    pub fn is_error_interrupt_enabled(&self) -> bool {
        (self.bits >> 3) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_half_transfer_interrupt_enabled(&self) -> bool {
        (self.bits >> 2) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_full_transfer_interrupt_enabled(&self) -> bool {
        (self.bits >> 1) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn is_chan_enabled(&self) -> bool {
        self.bits & 0x1 == 0x1
    }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn set_mem_to_mem(&mut self, value: bool) {
        if value {
            self.bits |= (1 << 14);
        } else {
            self.bits &= !(1 << 14);
        }
    }

    #[inline(always)]
    pub fn set_priority_level(&mut self, value: PriorityLevel) {
        self.bits = (self.bits & !(0x2 << 12)) | ((value.raw_value() as u32) << 12);
    }

    #[inline(always)]
    pub fn set_mem_transfer_size(&mut self, value: TransferSize) {
        self.bits = (self.bits & !(0x2 << 10)) | ((value.raw_value() as u32) << 10);
    }

    #[inline(always)]
    pub fn set_peri_transfer_size(&mut self, value: TransferSize) {
        self.bits = (self.bits & !(0x2 << 8)) | ((value.raw_value() as u32) << 8);
    }

    #[inline(always)]
    pub fn set_mem_next_address_gen_mode(&mut self, value: AddressGenMode) {
        match value {
            AddressGenMode::Fixed => {
                self.bits |= (1 << 7);
            }
            AddressGenMode::Increasing => {
                self.bits &= !(1 << 7);
            }
        }
    }

    #[inline(always)]
    pub fn set_peri_next_address_gen_mode(&mut self, value: AddressGenMode) {
        match value {
            AddressGenMode::Fixed => {
                self.bits |= (1 << 6);
            }
            AddressGenMode::Increasing => {
                self.bits &= !(1 << 6);
            }
        }
    }

    #[inline(always)]
    pub fn set_circular_mode_enabled(&mut self, value: bool) {
        if value {
            self.bits |= (1 << 5);
        } else {
            self.bits &= !(1 << 5);
        }
    }

    #[inline(always)]
    pub fn set_transfer_direction(&mut self, value: TransferDirection) {
        match value {
            TransferDirection::MemReadPeriWrite => {
                self.bits |= (1 << 4);
            }
            TransferDirection::PeriReadMemWrite => {
                self.bits &= !(1 << 4);
            }
        }
    }

    #[inline(always)]
    pub fn set_error_interrupt_enabled(&mut self, value: bool) {
        if value {
            self.bits |= (1 << 3);
        } else {
            self.bits &= !(1 << 3);
        }
    }

    #[inline(always)]
    pub fn set_half_transfer_interrupt_enabled(&mut self, value: bool) {
        if value {
            self.bits |= (1 << 2);
        } else {
            self.bits &= !(1 << 2);
        }
    }

    #[inline(always)]
    pub fn set_full_transfer_interrupt_enabled(&mut self, value: bool) {
        if value {
            self.bits |= (1 << 1);
        } else {
            self.bits &= !(1 << 1);
        }
    }

    #[inline(always)]
    pub fn set_chan_enabled(&mut self, value: bool) {
        if value {
            self.bits |= (1 << 0);
        } else {
            self.bits &= !(1 << 0);
        }
    }
}