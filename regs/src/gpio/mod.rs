pub mod ctl;

pub enum Block {
    A,
    B,
    C,
    D,
    E,
}

impl Block {
    const fn base_address(&self) -> u32 {
        match *self {
            Self::A => 0x4001_0800,
            Self::B => 0x4001_0C00,
            Self::C => 0x4001_1000,
            Self::D => 0x4001_1400,
            Self::E => 0x4001_1800,
        }
    }
}

pub enum Port {
    Port0,
    Port1,
    Port2,
    Port3,
    Port4,
    Port5,
    Port6,
    Port7,
    Port8,
    Port9,
    Port10,
    Port11,
    Port12,
    Port13,
    Port14,
    Port15,
}

impl Port {
    const fn raw_value(&self) -> u8 {
        match *self {
            Self::Port0 => 0,
            Self::Port1 => 1,
            Self::Port2 => 2,
            Self::Port3 => 3,
            Self::Port4 => 4,
            Self::Port5 => 5,
            Self::Port6 => 6,
            Self::Port7 => 7,
            Self::Port8 => 8,
            Self::Port9 => 9,
            Self::Port10 => 10,
            Self::Port11 => 11,
            Self::Port12 => 12,
            Self::Port13 => 13,
            Self::Port14 => 14,
            Self::Port15 => 15,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum PortConfiguration {
    InputAnalog_OutputGPIOPushPull,
    InputFloating_OutputGPIOOpenDrain,
    InputPullUpDown_OutputAFIOPushPull,
    InputReserved_OutputAFIOOpenDrain,
}

impl PortConfiguration {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::InputAnalog_OutputGPIOPushPull),
            0b01 => Some(Self::InputFloating_OutputGPIOOpenDrain),
            0b10 => Some(Self::InputPullUpDown_OutputAFIOPushPull),
            0b11 => Some(Self::InputReserved_OutputAFIOOpenDrain),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::InputAnalog_OutputGPIOPushPull => 0b00,
            Self::InputFloating_OutputGPIOOpenDrain => 0b01,
            Self::InputPullUpDown_OutputAFIOPushPull => 0b10,
            Self::InputReserved_OutputAFIOOpenDrain => 0b11,
        }
    }
}

pub enum PortMode {
    Input,
    Output10MHz,
    Output2MHz,
    Output50MHz,
}

impl PortMode {
    const fn try_from_raw_value(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::Input),
            0b01 => Some(Self::Output10MHz),
            0b10 => Some(Self::Output2MHz),
            0b11 => Some(Self::Output50MHz),
            _ => None
        }
    }

    const fn raw_value(&self) -> u8 {
        match *self {
            Self::Input => 0b00,
            Self::Output10MHz => 0b01,
            Self::Output2MHz => 0b10,
            Self::Output50MHz => 0b11,
        }
    }
}