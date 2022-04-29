#![no_std]
#![no_main]

fn main() {
    add_two(10, 11);
}

fn add_two(a:i32, b:i32) -> i32 {
    return a + b;
}


use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}