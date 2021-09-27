use crate::regs::dma::{Bank, Channel};

const OFFSET: u32 = 0x04;

#[inline(always)]
pub fn clear_transfer_error_flag(bank: Bank, channel: Channel) {
    set(bank, channel, 3);
}

#[inline(always)]
pub fn clear_half_transfer_finish_flag(bank: Bank, channel: Channel) {
    set(bank, channel, 2);
}

#[inline(always)]
pub fn clear_full_transfer_finish_flag(bank: Bank, channel: Channel) {
    set(bank, channel, 1);
}

#[inline(always)]
pub fn clear_global_interrupt_flag(bank: Bank, channel: Channel) {
    set(bank, channel, 0);
}

#[inline(always)]
fn set(bank: Bank, channel: Channel, offset: u32) {
    let address = (bank.base_address() + OFFSET) as *mut u32;
    let shift_by = (4 * channel.number() as u32) + offset;
    unsafe { *address = (1 << shift_by); }
}