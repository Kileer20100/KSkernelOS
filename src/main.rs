// src/main.rs
#![no_std]
#![no_main]

mod drivers;
mod set_macros;

use crate::drivers::vga::xd8000::{println, text::text_write};

use core::panic::PanicInfo;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //text_write();
    
    println!("Hello, KSkernelOS! 1");
    println_warn!("This is a warning message!");
    println_error!("This is an error message!");



    loop {
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

