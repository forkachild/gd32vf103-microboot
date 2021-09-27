use crate::regs::rcu::{BASE_ADDRESS, I2SClkSrcSel, PREDVInputClkSrcSel, PLLxSrcClkMulFactor, PREDVxInputSrcClkDivFactor};

const OFFSET: u32 = 0x2C;
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
    pub fn i2s2_clk_src_sel(&self) -> Option<I2SClkSrcSel> {
        I2SClkSrcSel::try_from_raw_value(((self.bits >> 18) & 0b1) as u8)
    }

    #[inline(always)]
    pub fn i2s1_clk_src_sel(&self) -> Option<I2SClkSrcSel> {
        I2SClkSrcSel::try_from_raw_value(((self.bits >> 17) & 0b1) as u8)
    }

    #[inline(always)]
    pub fn predv0_input_clk_src_sel(&self) -> Option<PREDVInputClkSrcSel> {
        PREDVInputClkSrcSel::try_from_raw_value(((self.bits >> 16) & 0b1) as u8)
    }

    #[inline(always)]
    pub fn pll2_clk_mul_factor(&self) -> Option<PLLxSrcClkMulFactor> {
        PLLxSrcClkMulFactor::try_from_raw_value(((self.bits >> 12) & 0b1111) as u8)
    }

    #[inline(always)]
    pub fn pll1_clk_mul_factor(&self) -> Option<PLLxSrcClkMulFactor> {
        PLLxSrcClkMulFactor::try_from_raw_value(((self.bits >> 8) & 0b1111) as u8)
    }

    #[inline(always)]
    pub fn predv1_input_src_clk_div_factor(&self) -> Option<PREDVxInputSrcClkDivFactor> {
        PREDVxInputSrcClkDivFactor::try_from_raw_value(((self.bits >> 4) & 0b1111) as u8)
    }

    #[inline(always)]
    pub fn predv0_input_src_clk_div_factor(&self) -> Option<PREDVxInputSrcClkDivFactor> {
        PREDVxInputSrcClkDivFactor::try_from_raw_value(((self.bits >> 0) & 0b1111) as u8)
    }
}

/// Mutation block
impl RegisterBlock {
    #[inline(always)]
    pub fn set_i2s2_clk_src_sel(&mut self, value: I2SClkSrcSel) {
        const POSITION: u32 = 18;
        match value {
            I2SClkSrcSel::SysClk => {
                self.bits |= (1 << POSITION);
            }
            I2SClkSrcSel::PLL2ClkMul2 => {
                self.bits &= !(1 << POSITION);
            }
        }
    }

    #[inline(always)]
    pub fn set_i2s1_clk_src_sel(&mut self, value: I2SClkSrcSel) {
        const POSITION: u32 = 17;
        match value {
            I2SClkSrcSel::SysClk => {
                self.bits |= (1 << POSITION);
            }
            I2SClkSrcSel::PLL2ClkMul2 => {
                self.bits &= !(1 << POSITION);
            }
        }
    }

    #[inline(always)]
    pub fn set_predv0_input_clk_src_sel(&mut self, value: PREDVInputClkSrcSel) {
        const POSITION: u32 = 16;
        match value {
            PREDVInputClkSrcSel::HXTAL => {
                self.bits |= (1 << POSITION);
            }
            PREDVInputClkSrcSel::PLL1Clk => {
                self.bits &= !(1 << POSITION);
            }
        }
    }

    #[inline(always)]
    pub fn set_pll2_clk_mul_factor(&mut self, value: PLLxSrcClkMulFactor) {
        const POSITION: u32 = 12;
        const MASK: u32 = 0b1111;
        self.bits = (self.bits & !(MASK << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn set_pll1_clk_mul_factor(&mut self, value: PLLxSrcClkMulFactor) {
        const POSITION: u32 = 8;
        const MASK: u32 = 0b1111;
        self.bits = (self.bits & !(MASK << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn set_predv1_input_src_clk_div_factor(&mut self, value: PREDVxInputSrcClkDivFactor) {
        const POSITION: u32 = 4;
        const MASK: u32 = 0b1111;
        self.bits = (self.bits & !(MASK << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn set_predv0_input_src_clk_div_factor(&mut self, value: PREDVxInputSrcClkDivFactor) {
        const POSITION: u32 = 0;
        const MASK: u32 = 0b1111;
        self.bits = (self.bits & !(MASK << POSITION)) | ((value.raw_value() as u32) << POSITION);
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.bits = RESET_VALUE;
    }
}