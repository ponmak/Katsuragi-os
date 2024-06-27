#![no_std]
#![no_main]
//import own test frameworks
#![feature(custom_test_frameworks)]
#![test_runner(katsuragi_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use katsuragi_os::println;
use katsuragi_os::print;

// no need because it used in lib file 
//use vga_buffer::WRITER;
//mod vga_buffer;
//mod serial;

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    katsuragi_os::test_panic_handler(info)
}

/* 
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running {} tests", tests.len());
    for test in tests{ //
    test.run();
}
exit_qemu(QemuExitCode::Success);
}
*/

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

    #[cfg(test)]
    test_main();
    //panic!("(!) hil kai");
    
    loop {}
}

/* 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    
    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    
}
*/

#[test_case]
fn trivial_assertion() {
    //print!("trivial assertion... ");
    assert_eq!(1, 1);
    //println!("(+) test complete");
}

/* 
//Insert Printing Automatically
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("(+) test complete");
    }
}

*/