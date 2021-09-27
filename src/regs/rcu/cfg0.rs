use crate::regs::rcu::{
    ADCClkPrescaler,
    AHBPrescaler,
    APBPrescaler,
    BASE_ADDRESS,
    ClkOutClkSrc,
    PLLClkMulFactor,
    PLLClkSrc,
    SysClkSwitch,
    USBFSClkPrescaler,
};

const OFFSET: u32 = 0x04;
const RESET_VALUE: u32 = 0x0000_0000;

#[repr(transparent)]
pub struct RegisterBlock {
    bits: u32,
}

#[inline(always)]
pub fn read() -> RegisterBlock {
    let address = (BASE_ADDRESS + OFFSET) as *const u32;
    RegisterBlock { bits: unsafe { *address } }
}

impl RegisterBlock {
    #[inline(always)]
    pub fn write_back(&self) {
        let address = (BASE_ADDRESS + OFFSET) as *mut u32;
        unsafe { *address = self.bits; }
    }
}

/// Query block
impl RegisterBlock {
    #[inline(always)]
    pub fn clk_out_clk_src(&self) -> Option<ClkOutClkSrc> {
        ClkOutClkSrc::try_from_raw_value(((self.bits >> 24) & 0xF) as u8)
    }

    #[inline(always)]
    pub fn usbfs_clk_prescaler(&self) -> Option<USBFSClkPrescaler> {
        USBFSClkPrescaler::try_from_raw_value(((self.bits >> 22) & 0x3) as u8)
    }

    #[inline(always)]
    pub fn pll_clk_mul_factor(&self) -> Option<PLLClkMulFactor> {
        const FIRST_BIT_POSITION: u32 = 29;
        const POSITION: u32 = 18;
        const SIZE: u32 = 4;
        PLLClkMulFactor::try_from_raw_value(
            (((self.bits >> (FIRST_BIT_POSITION - SIZE)) & (1 << SIZE)) | ((self.bits >> POSITION) & 0xF)) as u8
        )
    }

    #[inline(always)]
    pub fn predv0_lsb(&self) -> bool {
        (self.bits >> 17) & 0x1 == 0x1
    }

    #[inline(always)]
    pub fn pll_clk_src(&self) -> Option<PLLClkSrc> {
        PLLClkSrc::try_from_raw_value(((self.bits >> 16) & 0x1) as u8)
    }

    #[inline(always)]
    pub fn adc_clk_prescaler(&self) -> Option<ADCClkPrescaler> {
        const FIRST_BIT_POSITION: u32 = 28;
        const POSITION: u32 = 14;
        const SIZE: u32 = 2;
        ADCClkPrescaler::try_from_raw_value(
            (((self.bits >> (FIRST_BIT_POSITION - SIZE)) & (1 << SIZE)) | ((self.bits >> POSITION) & 0x2)) as u8
        )
    }

    #[inline(always)]
    pub fn apb2_prescaler(&self) -> Option<APBPrescaler> {
        APBPrescaler::try_from_raw_value(((self.bits >> 11) & 0x7) as u8)
    }

    #[inline(always)]
    pub fn apb1_prescaler(&self) -> Option<APBPrescaler> {
        APBPrescaler::try_from_raw_value(((self.bits >> 8) & 0x7) as u8)
    }

    #[inline(always)]
    pub fn ahb_prescaler(&self) -> Option<AHBPrescaler> {
        AHBPrescaler::try_from_raw_value(((self.bits >> 4) & 0xF) as u8)
    }

    #[inline(always)]
    pub fn sys_clk_switch_status(&self) -> Option<SysClkSwitch> {
        SysClkSwitch::try_from_raw_value(((self.bits >> 2) & 0x3) as u8)
    }

    #[inline(always)]
    pub fn sys_clk_switch(&self) -> Option<SysClkSwitch> {
        SysClkSwitch::try_from_raw_value((self.bits & 0x3) as u8)
    }
}

/// Mutation block
impl RegisterBlock {
    #[inline(always)]
    pub fn set_clk_out_clk_src(&mut self, value: ClkOutClkSrc) {
        const POSITION: u32 = 24;
        self.bits = (self.bits & !(0xF << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn set_usbfs_clk_prescaler(&mut self, value: USBFSClkPrescaler) {
        const POSITION: u32 = 22;
        self.bits = (self.bits & !(0x3 << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn set_pll_clk_mul_factor(&mut self, value: PLLClkMulFactor) {
        const FIRST_BIT_POSITION: u32 = 29;
        const POSITION: u32 = 18;
        const SIZE: u32 = 4;
        let value = value.raw_value() as u32;
        self.bits = (self.bits & !((1 << FIRST_BIT_POSITION) | (0xF << POSITION)))
            | ((value & 0x10) << (FIRST_BIT_POSITION - SIZE)) as u32
            | ((value & 0xF) << POSITION) as u32;
    }

    #[inline(always)]
    pub fn set_predv0_lsb(&mut self, value: bool) {
        const POSITION: u32 = 17;
        if value {
            self.bits |= (1 << POSITION);
        } else {
            self.bits &= !(1 << POSITION);
        }
    }

    #[inline(always)]
    pub fn set_pll_clk_src(&mut self, value: PLLClkSrc) {
        const POSITION: u32 = 16;
        match value {
            PLLClkSrc::Internal => {
                self.bits |= (1 << POSITION);
            }
            PLLClkSrc::HXTAL => {
                self.bits &= !(1 << POSITION);
            }
        }
    }

    #[inline(always)]
    pub fn set_adc_clk_prescaler(&mut self, value: ADCClkPrescaler) {
        const FIRST_BIT_POSITION: u32 = 28;
        const POSITION: u32 = 14;
        const SIZE: u32 = 2;
        let value = value.raw_value() as u32;
        self.bits = (self.bits & !((1 << FIRST_BIT_POSITION) | (0xF << POSITION)))
            | ((value & 0x10) << (FIRST_BIT_POSITION - SIZE)) as u32
            | ((value & 0xF) << POSITION) as u32;
    }

    #[inline(always)]
    pub fn set_apb2_prescaler(&mut self, value: APBPrescaler) {
        const POSITION: u32 = 11;
        self.bits = (self.bits & !(0x7 << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn set_apb1_prescaler(&mut self, value: APBPrescaler) {
        const POSITION: u32 = 8;
        self.bits = (self.bits & !(0x7 << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn set_ahb_prescaler(&mut self, value: AHBPrescaler) {
        const POSITION: u32 = 4;
        self.bits = (self.bits & !(0xF << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn set_sys_clk_switch(&mut self, value: SysClkSwitch) {
        const POSITION: u32 = 0;
        self.bits = (self.bits & !(0x3 << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.bits = RESET_VALUE;
    }
}