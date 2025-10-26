
use crate::drivers::vga::xd8000::color::{Color};
use core::fmt::Write;

static mut SKREEN_TEXT: [u8; 80*25*2] = [0; 80*25*2];
static mut ITER: u32 = 0;

pub fn println(agrs: core::fmt::Arguments) {
    let mut printer = VGAPrinter;
    core::fmt::Write::write_fmt(&mut printer, agrs).unwrap();
}

struct VGAPrinter;

impl Write for VGAPrinter {
    fn write_str(&mut self, message: &str) -> core::fmt::Result {

                let vga_buffer = 0xb8000 as *mut u8;
                let text = message.bytes().enumerate();

                for (i, byte) in text{
                    unsafe {
                        *vga_buffer.offset(i as isize * 2) = byte;
                        *vga_buffer.offset(i as isize * 2 + 1) = Color::text_write();
                    }
                }
            Ok(())
        }
}

fn vga_buffer(){
    
}