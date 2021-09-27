macro_rules! bulk_gen_bit_access {
    ($reg_block:ident,$(($fn_name:ident;$position:literal)),+) => {
        impl $reg_block {
            $(
            #[inline(always)]
            pub fn $fn_name(&self) -> bool {
                (self.bits >> $position) & 0b1 == 0b1
            }
            )+
        }
    }
}

macro_rules! bulk_gen_bit_modify {
    ($reg_block:ident,$(($fn_name:ident;$position:literal)),+) => {
        impl $reg_block {
            $(
            #[inline(always)]
            pub fn $fn_name(&mut self, value: bool) {
                const POSITION: u32 = $position;
                if value {
                    self.bits |= (1 << POSITION);
                } else {
                    self.bits &= !(1 << POSITION);
                }
            }
            )+
        }
    }
}