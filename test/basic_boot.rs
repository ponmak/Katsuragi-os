
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(katsuragi_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use katsuragi_os::println;

// making separate executables
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(test: &[&dyn Fn()]){
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    katsuragi_os::panic_handler(info)
}


#[test_case]
fn test_println() {
    print!("(+) test_println output");
}