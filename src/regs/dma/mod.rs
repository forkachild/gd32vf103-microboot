pub mod intf;
pub mod intc;
pub mod chctl;
pub mod chcnt;
pub mod chpaddr;
pub mod chmaddr;

pub enum Bank {
    DMA0,
    DMA1,
}

impl Bank {
    const fn base_address(&self) -> u32 {
        match self {
            Bank::DMA0 => 0x4002_0000,
            Bank::DMA1 => 0x4002_0400,
        }
    }
}

pub enum Channel {
    Chan0,
    Chan1,
    Chan2,
    Chan3,
    Chan4,
    Chan5,
    Chan6,
}

impl Channel {
    const fn number(&self) -> u8 {
        match self {
            Channel::Chan0 => 0,
            Channel::Chan1 => 1,
            Channel::Chan2 => 2,
            Channel::Chan3 => 3,
            Channel::Chan4 => 4,
            Channel::Chan5 => 5,
            Channel::Chan6 => 6
        }
    }
}

pub enum PriorityLevel {
    Low,
    Medium,
    High,
    UltraHigh,
}

impl PriorityLevel {
    const fn from_value(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::Low),
            0b01 => Some(Self::Medium),
            0b10 => Some(Self::High),
            0b11 => Some(Self::UltraHigh),
            _ => None
        }
    }
}

impl PriorityLevel {
    const fn raw_value(&self) -> u8 {
        match self {
            Self::Low => 0b00,
            Self::Medium => 0b01,
            Self::High => 0b10,
            Self::UltraHigh => 0b11
        }
    }
}

pub enum TransferSize {
    Bits8,
    Bits16,
    Bits32,
}

impl TransferSize {
    const fn from_value(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::Bits8),
            0b01 => Some(Self::Bits16),
            0b10 => Some(Self::Bits32),
            _ => None
        }
    }
}

impl TransferSize {
    const fn raw_value(&self) -> u8 {
        match self {
            Self::Bits8 => 0b00,
            Self::Bits16 => 0b01,
            Self::Bits32 => 0b10
        }
    }
}

pub enum AddressGenMode {
    Fixed,
    Increasing,
}

impl AddressGenMode {
    const fn from_value(value: u8) -> Option<Self> {
        match value {
            0b0 => Some(Self::Fixed),
            0b1 => Some(Self::Increasing),
            _ => None
        }
    }
}

impl AddressGenMode {
    const fn raw_value(&self) -> u8 {
        match self {
            Self::Fixed => 0b0,
            Self::Increasing => 0b1,
        }
    }
}

pub enum TransferDirection {
    PeriReadMemWrite,
    MemReadPeriWrite,
}

impl TransferDirection {
    const fn from_value(value: u8) -> Option<Self> {
        match value {
            0b0 => Some(Self::PeriReadMemWrite),
            0b1 => Some(Self::MemReadPeriWrite),
            _ => None
        }
    }
}

impl TransferDirection {
    const fn raw_value(&self) -> u8 {
        match self {
            Self::PeriReadMemWrite => 0b0,
            Self::MemReadPeriWrite => 0b1
        }
    }
}