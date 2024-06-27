#![no_std]
#![no_main]

use core::panic::PanicInfo;
use katsuragi_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("(+) should panic ok");
    exit_qemu(QemuExitCode::Success);

    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("(-) test did not panic!");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn should_fail() {
    serial_print!("(!) should_panic::should_fail...\t");
    assert_eq!(0, 1);
}