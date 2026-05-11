#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

mod arch;
mod uart;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    uart::write_line("Hello");

    loop {
        arch::cpu::wait_for_event();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
