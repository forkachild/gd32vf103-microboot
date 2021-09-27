pub mod ctl;
pub mod cfg0;
pub mod int;
pub mod apb1;
pub mod apb2;
pub mod ahb;
pub mod bdctl;
pub mod rstsck;
pub mod cfg1;
pub mod dsv;

const BASE_ADDRESS: u32 = 0x4002_1000;

pub enum ClkOutClkSrc {
    None,
    System,
    HighSpeedInternalOsc,
    ExternalHighSpeedOsc,
    PLLDiv2,
    PLL1,
    PLL2Div2,
    EXT1,
    PLL2,
}

impl ClkOutClkSrc {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b0000..=0b0011 => Some(Self::None),
            0b0100 => Some(Self::System),
            0b0101 => Some(Self::HighSpeedInternalOsc),
            0b0110 => Some(Self::ExternalHighSpeedOsc),
            0b0111 => Some(Self::PLLDiv2),
            0b1000 => Some(Self::PLL1),
            0b1001 => Some(Self::PLL2Div2),
            0b1010 => Some(Self::EXT1),
            0b1011 => Some(Self::PLL2),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::None => 0b0000,
            Self::System => 0b0100,
            Self::HighSpeedInternalOsc => 0b0101,
            Self::ExternalHighSpeedOsc => 0b0110,
            Self::PLLDiv2 => 0b0111,
            Self::PLL1 => 0b1000,
            Self::PLL2Div2 => 0b1001,
            Self::EXT1 => 0b1010,
            Self::PLL2 => 0b1011,
        }
    }
}

pub enum USBFSClkPrescaler {
    PLLDiv1_5,
    PLL,
    PLLDiv2_5,
    PLLDiv2,
}

impl USBFSClkPrescaler {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::PLLDiv1_5),
            0b01 => Some(Self::PLL),
            0b10 => Some(Self::PLLDiv2_5),
            0b11 => Some(Self::PLLDiv2),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::PLLDiv1_5 => 0b00,
            Self::PLL => 0b01,
            Self::PLLDiv2_5 => 0b10,
            Self::PLLDiv2 => 0b11
        }
    }
}

pub enum PLLClkMulFactor {
    PLLMul2,
    PLLMul3,
    PLLMul4,
    PLLMul5,
    PLLMul6,
    PLLMul7,
    PLLMul8,
    PLLMul9,
    PLLMul10,
    PLLMul11,
    PLLMul12,
    PLLMul13,
    PLLMul14,
    PLLMul6_5,
    PLLMul16,
    PLLMul17,
    PLLMul18,
    PLLMul19,
    PLLMul20,
    PLLMul21,
    PLLMul22,
    PLLMul23,
    PLLMul24,
    PLLMul25,
    PLLMul26,
    PLLMul27,
    PLLMul28,
    PLLMul29,
    PLLMul30,
    PLLMul31,
    PLLMul32,
}

impl PLLClkMulFactor {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b00000 => Some(Self::PLLMul2),
            0b00001 => Some(Self::PLLMul3),
            0b00010 => Some(Self::PLLMul4),
            0b00011 => Some(Self::PLLMul5),
            0b00100 => Some(Self::PLLMul6),
            0b00101 => Some(Self::PLLMul7),
            0b00110 => Some(Self::PLLMul8),
            0b00111 => Some(Self::PLLMul9),
            0b01000 => Some(Self::PLLMul10),
            0b01001 => Some(Self::PLLMul11),
            0b01010 => Some(Self::PLLMul12),
            0b01011 => Some(Self::PLLMul13),
            0b01100 => Some(Self::PLLMul14),
            0b01101 => Some(Self::PLLMul6_5),
            0b01110 | 0b01111 => Some(Self::PLLMul16),
            0b10000 => Some(Self::PLLMul17),
            0b10001 => Some(Self::PLLMul18),
            0b10010 => Some(Self::PLLMul19),
            0b10011 => Some(Self::PLLMul20),
            0b10100 => Some(Self::PLLMul21),
            0b10101 => Some(Self::PLLMul22),
            0b10110 => Some(Self::PLLMul23),
            0b10111 => Some(Self::PLLMul24),
            0b11000 => Some(Self::PLLMul25),
            0b11001 => Some(Self::PLLMul26),
            0b11010 => Some(Self::PLLMul27),
            0b11011 => Some(Self::PLLMul28),
            0b11100 => Some(Self::PLLMul29),
            0b11101 => Some(Self::PLLMul30),
            0b11110 => Some(Self::PLLMul31),
            0b11111 => Some(Self::PLLMul32),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::PLLMul2 => 0b00000,
            Self::PLLMul3 => 0b00001,
            Self::PLLMul4 => 0b00010,
            Self::PLLMul5 => 0b00011,
            Self::PLLMul6 => 0b00100,
            Self::PLLMul7 => 0b00101,
            Self::PLLMul8 => 0b00110,
            Self::PLLMul9 => 0b00111,
            Self::PLLMul10 => 0b01000,
            Self::PLLMul11 => 0b01001,
            Self::PLLMul12 => 0b01010,
            Self::PLLMul13 => 0b01011,
            Self::PLLMul14 => 0b01100,
            Self::PLLMul6_5 => 0b01101,
            Self::PLLMul16 => 0b01110,
            Self::PLLMul17 => 0b10000,
            Self::PLLMul18 => 0b10001,
            Self::PLLMul19 => 0b10010,
            Self::PLLMul20 => 0b10011,
            Self::PLLMul21 => 0b10100,
            Self::PLLMul22 => 0b10101,
            Self::PLLMul23 => 0b10110,
            Self::PLLMul24 => 0b10111,
            Self::PLLMul25 => 0b11000,
            Self::PLLMul26 => 0b11001,
            Self::PLLMul27 => 0b11010,
            Self::PLLMul28 => 0b11011,
            Self::PLLMul29 => 0b11100,
            Self::PLLMul30 => 0b11101,
            Self::PLLMul31 => 0b11110,
            Self::PLLMul32 => 0b11111,
        }
    }
}

pub enum PLLClkSrc {
    Internal,
    HXTAL,
}

impl PLLClkSrc {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b0 => Some(Self::Internal),
            0b1 => Some(Self::HXTAL),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::Internal => 0b0,
            Self::HXTAL => 0b1,
        }
    }
}

pub enum ADCClkPrescaler {
    APB2ClkDiv2,
    APB2ClkDiv4,
    APB2ClkDiv6,
    APB2ClkDiv8,
    APB2ClkDiv12,
    APB2ClkDiv16,
}

impl ADCClkPrescaler {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b000 | 0b100 => Some(Self::APB2ClkDiv2),
            0b001 => Some(Self::APB2ClkDiv4),
            0b010 => Some(Self::APB2ClkDiv6),
            0b011 | 0b110 => Some(Self::APB2ClkDiv8),
            0b101 => Some(Self::APB2ClkDiv12),
            0b111 => Some(Self::APB2ClkDiv16),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::APB2ClkDiv2 => 0b000,
            Self::APB2ClkDiv4 => 0b001,
            Self::APB2ClkDiv6 => 0b010,
            Self::APB2ClkDiv8 => 0b011,
            Self::APB2ClkDiv12 => 0b101,
            Self::APB2ClkDiv16 => 0b111,
        }
    }
}

pub enum APBPrescaler {
    AHBClk,
    AHBClkDiv2,
    AHBClkDiv4,
    AHBClkDiv8,
    AHBClkDiv16,
}

impl APBPrescaler {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b000..=0b011 => Some(Self::AHBClk),
            0b100 => Some(Self::AHBClkDiv2),
            0b101 => Some(Self::AHBClkDiv4),
            0b110 => Some(Self::AHBClkDiv8),
            0b111 => Some(Self::AHBClkDiv16),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::AHBClk => 0b000,
            Self::AHBClkDiv2 => 0b100,
            Self::AHBClkDiv4 => 0b101,
            Self::AHBClkDiv8 => 0b110,
            Self::AHBClkDiv16 => 0b111,
        }
    }
}

pub enum AHBPrescaler {
    SysClk,
    SysClkDiv2,
    SysClkDiv4,
    SysClkDiv8,
    SysClkDiv16,
    SysClkDiv64,
    SysClkDiv128,
    SysClkDiv256,
    SysClkDiv512,
}

impl AHBPrescaler {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b0000..=0b0111 => Some(Self::SysClk),
            0b1000 => Some(Self::SysClkDiv2),
            0b1001 => Some(Self::SysClkDiv4),
            0b1010 => Some(Self::SysClkDiv8),
            0b1011 => Some(Self::SysClkDiv16),
            0b1100 => Some(Self::SysClkDiv64),
            0b1101 => Some(Self::SysClkDiv128),
            0b1110 => Some(Self::SysClkDiv256),
            0b1111 => Some(Self::SysClkDiv512),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::SysClk => 0b0000,
            Self::SysClkDiv2 => 0b1000,
            Self::SysClkDiv4 => 0b1001,
            Self::SysClkDiv8 => 0b1010,
            Self::SysClkDiv16 => 0b1011,
            Self::SysClkDiv64 => 0b1100,
            Self::SysClkDiv128 => 0b1101,
            Self::SysClkDiv256 => 0b1110,
            Self::SysClkDiv512 => 0b1111,
        }
    }
}

pub enum SysClkSwitch {
    InternalRC8MClk,
    HXTALClk,
    PLLClk,
}

impl SysClkSwitch {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::InternalRC8MClk),
            0b01 => Some(Self::HXTALClk),
            0b10 => Some(Self::PLLClk),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::InternalRC8MClk => 0b00,
            Self::HXTALClk => 0b01,
            Self::PLLClk => 0b10,
        }
    }
}

pub enum RTCClkEntrySel {
    NoClk,
    LXTAL,
    IRC_40K,
    HXTALDiv128,
}

impl RTCClkEntrySel {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::NoClk),
            0b01 => Some(Self::LXTAL),
            0b10 => Some(Self::IRC_40K),
            0b11 => Some(Self::HXTALDiv128),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            RTCClkEntrySel::NoClk => 0b00,
            RTCClkEntrySel::LXTAL => 0b01,
            RTCClkEntrySel::IRC_40K => 0b10,
            RTCClkEntrySel::HXTALDiv128 => 0b11,
        }
    }
}

pub enum I2SClkSrcSel {
    SysClk,
    PLL2ClkMul2,
}

impl I2SClkSrcSel {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b0 => Some(Self::SysClk),
            0b1 => Some(Self::PLL2ClkMul2),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::SysClk => 0b0,
            Self::PLL2ClkMul2 => 0b1,
        }
    }
}

pub enum PREDVInputClkSrcSel {
    HXTAL,
    PLL1Clk,
}

impl PREDVInputClkSrcSel {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b0 => Some(Self::HXTAL),
            0b1 => Some(Self::PLL1Clk),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::HXTAL => 0b0,
            Self::PLL1Clk => 0b1,
        }
    }
}

pub enum PLLxSrcClkMulFactor {
    Mul8,
    Mul9,
    Mul10,
    Mul11,
    Mul12,
    Mul13,
    Mul14,
    Mul15,
    Mul16,
    Mul20,
}

impl PLLxSrcClkMulFactor {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b0110 => Some(Self::Mul8),
            0b0111 => Some(Self::Mul9),
            0b1000 => Some(Self::Mul10),
            0b1001 => Some(Self::Mul11),
            0b1010 => Some(Self::Mul12),
            0b1011 => Some(Self::Mul13),
            0b1100 => Some(Self::Mul14),
            0b1101 => Some(Self::Mul15),
            0b1110 => Some(Self::Mul16),
            0b1111 => Some(Self::Mul20),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::Mul8 => 0b0110,
            Self::Mul9 => 0b0111,
            Self::Mul10 => 0b1000,
            Self::Mul11 => 0b1001,
            Self::Mul12 => 0b1010,
            Self::Mul13 => 0b1011,
            Self::Mul14 => 0b1100,
            Self::Mul15 => 0b1101,
            Self::Mul16 => 0b1110,
            Self::Mul20 => 0b1111,
        }
    }
}

pub enum PREDVxInputSrcClkDivFactor {
    NoDiv,
    Div2,
    Div3,
    Div4,
    Div5,
    Div6,
    Div7,
    Div8,
    Div9,
    Div10,
    Div11,
    Div12,
    Div13,
    Div14,
    Div15,
    Div16,
}

impl PREDVxInputSrcClkDivFactor {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b0000 => Some(Self::NoDiv),
            0b0001 => Some(Self::Div2),
            0b0010 => Some(Self::Div3),
            0b0011 => Some(Self::Div4),
            0b0100 => Some(Self::Div5),
            0b0101 => Some(Self::Div6),
            0b0110 => Some(Self::Div7),
            0b0111 => Some(Self::Div8),
            0b1000 => Some(Self::Div9),
            0b1001 => Some(Self::Div10),
            0b1010 => Some(Self::Div11),
            0b1011 => Some(Self::Div12),
            0b1100 => Some(Self::Div13),
            0b1101 => Some(Self::Div14),
            0b1110 => Some(Self::Div15),
            0b1111 => Some(Self::Div16),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::NoDiv => 0b0000,
            Self::Div2 => 0b0001,
            Self::Div3 => 0b0010,
            Self::Div4 => 0b0011,
            Self::Div5 => 0b0100,
            Self::Div6 => 0b0101,
            Self::Div7 => 0b0110,
            Self::Div8 => 0b0111,
            Self::Div9 => 0b1000,
            Self::Div10 => 0b1001,
            Self::Div11 => 0b1010,
            Self::Div12 => 0b1011,
            Self::Div13 => 0b1100,
            Self::Div14 => 0b1101,
            Self::Div15 => 0b1110,
            Self::Div16 => 0b1111,
        }
    }
}

pub enum CoreVoltageSel {
    V1_2,
    V1_1,
    V1_0,
    V0_9,
}

impl CoreVoltageSel {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::V1_2),
            0b01 => Some(Self::V1_1),
            0b10 => Some(Self::V1_0),
            0b11 => Some(Self::V0_9),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::V1_2 => 0b00,
            Self::V1_1 => 0b01,
            Self::V1_0 => 0b10,
            Self::V0_9 => 0b11,
        }
    }
}