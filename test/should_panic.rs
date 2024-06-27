#![no_std]
#![no_main]

use core::panic::PanicInfo;
use katsuragi_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("(+) should panic ok");
    exit_qemu(QemuExitCode::Success);

    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("(+) test didn't panic");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[test_case]
fn should_fail() {
    serial_print!("(!) should_panic::should_fail...\t");
    assert_eq!(0, 1);
}