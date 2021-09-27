#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod memory;
mod boot;
mod regs;
mod alloc;

fn main() -> ! {
    loop {}
}