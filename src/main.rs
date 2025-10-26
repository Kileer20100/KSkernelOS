// src/main.rs
#![no_std]
#![no_main]

mod drivers;
mod set_macros;

use crate::drivers::vga::xd8000::text::text_write;

use core::panic::PanicInfo;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //text_write();

    println!("Hello, VGA World! {}", 2);
    loop {
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

