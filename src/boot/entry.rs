use core::mem::zeroed;
use core::panic::PanicInfo;
use core::ptr::{copy_nonoverlapping, write_bytes};

use crate::main;

extern "C" {
    static __sdata: usize;
    static __edata: usize;
    static __sidata: usize;
    static __sbss: usize;
    static __ebss: usize;
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    init_ram();
    main();
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[inline(always)]
fn init_ram() {
    let data_size = unsafe { __edata - __sdata };
    let data_start_in_flash = unsafe { __sidata as *mut usize };
    let data_start_in_ram = unsafe { __sdata as *mut usize };
    unsafe { copy_nonoverlapping(data_start_in_flash, data_start_in_ram, data_size) };

    let bss_size = unsafe { __ebss - __sbss };
    let bss_start_in_ram = unsafe { __sbss as *mut usize };
    unsafe { write_bytes(bss_start_in_ram, 0, bss_size) };
}