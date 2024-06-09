#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

//static HELLO: &[u8] = b"Hello World!";
// this function is the entry point, since the linker looks for a function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //now useless
    //vga_buffer::print_somthing();
    
    /*use core::fmt::Write;
    vga_buffer::WRITER.lock().wirte_str("").unwrap();
    write!(vga_buffer::WRITER.lock(), "{}",42.6969).unwarp();*/

    println!("Welcome to KatsuragiOS");
    panic!("(!) hil kai");
    loop {}
}