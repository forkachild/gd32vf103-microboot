pub mod chcnt;
pub mod chctl;
pub mod chmaddr;
pub mod chpaddr;
pub mod intc;
pub mod intf;

pub enum Bank {
    Bank0,
    Bank1,
}

impl Bank {
    pub fn base_address(&self) -> usize {
        match *self {
            Self::Bank0 => 0x4002_0000,
            Self::Bank1 => 0x4002_1000,
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
    pub fn offset(&self) -> usize {
        match *self {
            Self::Chan0 => 0x14 * 0,
            Self::Chan1 => 0x14 * 1,
            Self::Chan2 => 0x14 * 2,
            Self::Chan3 => 0x14 * 3,
            Self::Chan4 => 0x14 * 4,
            Self::Chan5 => 0x14 * 5,
            Self::Chan6 => 0x14 * 6,
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
    const fn from_value(value: usize) -> Option<Self> {
        match value {
            0b00 => Some(Self::Low),
            0b01 => Some(Self::Medium),
            0b10 => Some(Self::High),
            0b11 => Some(Self::UltraHigh),
            _ => None,
        }
    }
}

impl PriorityLevel {
    const fn raw_value(&self) -> usize {
        match self {
            Self::Low => 0b00,
            Self::Medium => 0b01,
            Self::High => 0b10,
            Self::UltraHigh => 0b11,
        }
    }
}

pub enum TransferSize {
    Bits8,
    Bits16,
    Bits32,
}

impl TransferSize {
    const fn from_value(value: usize) -> Option<Self> {
        match value {
            0b00 => Some(Self::Bits8),
            0b01 => Some(Self::Bits16),
            0b10 => Some(Self::Bits32),
            _ => None,
        }
    }
}

impl TransferSize {
    const fn raw_value(&self) -> usize {
        match self {
            Self::Bits8 => 0b00,
            Self::Bits16 => 0b01,
            Self::Bits32 => 0b10,
        }
    }
}

pub enum AddressGenMode {
    Fixed,
    Increasing,
}

impl AddressGenMode {
    const fn from_value(value: usize) -> Option<Self> {
        match value {
            0b0 => Some(Self::Fixed),
            0b1 => Some(Self::Increasing),
            _ => None,
        }
    }
}

impl AddressGenMode {
    const fn raw_value(&self) -> usize {
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
    const fn from_value(value: usize) -> Option<Self> {
        match value {
            0b0 => Some(Self::PeriReadMemWrite),
            0b1 => Some(Self::MemReadPeriWrite),
            _ => None,
        }
    }
}

impl TransferDirection {
    const fn raw_value(&self) -> usize {
        match self {
            Self::PeriReadMemWrite => 0b0,
            Self::MemReadPeriWrite => 0b1,
        }
    }
}
