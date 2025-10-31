// src/main.rs
#![no_std]
#![no_main]

pub mod drivers;
pub mod set_macros;
pub mod task;
pub mod experiments;
pub mod memory;


extern crate alloc;



#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();



use linked_list_allocator::LockedHeap;
//use crate::drivers::vga::xd8000::{println, text::text_write};
use core::panic::PanicInfo;




#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {



    //text_write();
    
    println!("Hello, KSkernelOS! {}", 1);
    println_warn!("This is a warning message!");
    println_error!("This is an error message! {}", 1);


    loop {
        
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
