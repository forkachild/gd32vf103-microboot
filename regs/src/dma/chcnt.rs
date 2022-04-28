use crate::dma::{Bank, Channel};

const OFFSET: usize = 0x0C;

// create_register!(ChCnt, usize);
// register_default_init!(ChCnt);
// register_read_access!(ChCnt, usize, (bank: Bank, channel: Channel), bank.base_address() + channel.offset() + OFFSET);
// register_write_access!(ChCnt, usize, (bank: Bank, channel: Channel), bank.base_address() + channel.offset() + OFFSET);
//
// impl ChCnt {
//     #[inline(always)]
//     pub fn count(&self) -> usize {
//         self.bits & bit_mask!(16)
//     }
// }
//
// impl ChCnt {
//     #[inline(always)]
//     pub fn set_count(&mut self, value: usize) {
//         self.bits = value & bit_mask!(16);
//     }
// }
