#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::print_something;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_something();

    loop {}
}
