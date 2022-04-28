use core::panic::PanicInfo;

use crate::alloc::alloc_init;
use crate::startup::abort;

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    alloc_init();
}

#[panic_handler]
pub fn panic_handler(_panic: &PanicInfo<'_>) -> ! {
    unsafe { abort() };
}
