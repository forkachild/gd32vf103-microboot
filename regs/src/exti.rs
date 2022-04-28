pub mod even;
pub mod ften;
pub mod inten;
pub mod pd;
pub mod rten;
pub mod swiev;

const BASE_ADDRESS: usize = 0x4001_0400;

pub struct EXTIConfig;

impl EXTIConfig {
    fn base_address() -> usize {
        BASE_ADDRESS
    }
}

pub enum EXTILine {
    Line0,
    Line1,
    Line2,
    Line3,
    Line4,
    Line5,
    Line6,
    Line7,
    Line8,
    Line9,
    Line10,
    Line11,
    Line12,
    Line13,
    Line14,
    Line15,
    Line16,
    Line17,
    Line18,
}

impl EXTILine {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Line0),
            1 => Some(Self::Line1),
            2 => Some(Self::Line2),
            3 => Some(Self::Line3),
            4 => Some(Self::Line4),
            5 => Some(Self::Line5),
            6 => Some(Self::Line6),
            7 => Some(Self::Line7),
            8 => Some(Self::Line8),
            9 => Some(Self::Line9),
            10 => Some(Self::Line10),
            11 => Some(Self::Line11),
            12 => Some(Self::Line12),
            13 => Some(Self::Line13),
            14 => Some(Self::Line14),
            15 => Some(Self::Line15),
            16 => Some(Self::Line16),
            17 => Some(Self::Line17),
            18 => Some(Self::Line18),
            _ => None,
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::Line0 => 0,
            Self::Line1 => 1,
            Self::Line2 => 2,
            Self::Line3 => 3,
            Self::Line4 => 4,
            Self::Line5 => 5,
            Self::Line6 => 6,
            Self::Line7 => 7,
            Self::Line8 => 8,
            Self::Line9 => 9,
            Self::Line10 => 10,
            Self::Line11 => 11,
            Self::Line12 => 12,
            Self::Line13 => 13,
            Self::Line14 => 14,
            Self::Line15 => 15,
            Self::Line16 => 16,
            Self::Line17 => 17,
            Self::Line18 => 18,
        }
    }
}
