#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";
// this function is the entry point, since the linker looks for a function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_somthing();
    
    loop {}
}