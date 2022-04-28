#![cfg_attr(not(test), no_std)]
#![feature(asm_const)]

pub mod csr;
pub mod accessor;
pub mod rcu;
pub mod exti;
mod dma;