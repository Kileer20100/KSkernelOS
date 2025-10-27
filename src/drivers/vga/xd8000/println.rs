
use crate::drivers::vga::xd8000::color::{Color};
use core::fmt::Write;

static mut SKREEN_TEXT: [u8; 25*80] = [0; 25*80];
static mut SKREEN_COLOR: [u8; 25*80] = [0; 25*80];

static mut ITER: u32 = 0;
static mut ROW_WRITE: u32 = 0;

pub fn println(agrs: core::fmt::Arguments) {
    let mut printer = VGAPrinter;
    core::fmt::Write::write_fmt(&mut printer, agrs).unwrap();
}

struct VGAPrinter;

impl Write for VGAPrinter {
    fn write_str(&mut self, message: &str) -> core::fmt::Result {
            //let text = message.bytes().enumerate();
            bufer_vga(message, Color::text_write());

            
            let base = 0xb8000 as *mut u8;
            let global_iteration = unsafe { ITER };
            let row = global_iteration / 80;
            let vga_buffer = unsafe {
                if global_iteration == 0 {
                    0xb8000 as *mut u8
                } else {
                    base.add((row as usize) * 0xa0)
                }
            };


            unsafe {
            let (start, stop) = slise_buffer_row();
            let slise_text_buffer = &SKREEN_TEXT[start..stop];
            let slise_color_buffer = &SKREEN_COLOR[start..stop];

                for i in 0..80{
                    
                    *vga_buffer.offset(i as isize * 2) = slise_text_buffer[i];
                    *vga_buffer.offset(i as isize * 2 + 1) = slise_color_buffer[i];
                    
                }
            Ok(())
        }
    }
}

fn slise_buffer_row() -> (usize, usize) {
    let global_iteration = unsafe { ITER };
    let row = global_iteration / 80;

    ((80 * row) as usize, (80 * (row + 1)) as usize)
}

fn global_iteration_() {
    let global_iteration = unsafe { ITER };


    let row = global_iteration / 80;

    unsafe {
        let row_write: u32;

        if row == 0 {
            let logick_slise = &SKREEN_TEXT[0..2];

            if logick_slise[0] != 0 && logick_slise[1] != 0 {
                row_write = 1;
            } else {
                row_write = 0;
            }
        } else if row > 0 && row < 24 {
            row_write = row + 1;
        } else {
            clear_buffer();
            row_write = 0;
        }

        ROW_WRITE = row_write;
    }
}

fn clear_buffer() {
    unsafe {
        for i in 0..(25 * 80) {
            SKREEN_TEXT[i] = 0;
            SKREEN_COLOR[i] = 0;
        }

        
        let base = 0xb8000 as *mut u8;
        for i in 0..(25 * 80) {
            *base.add(i * 2) = b' ';
            *base.add(i * 2 + 1) = Color::BLACK;
        }
    }
}


fn bufer_vga(message: &str, color: u8) {
    let text = message.as_bytes();

    global_iteration_();

    let start_base = unsafe { (ROW_WRITE as usize) * 80 };

    unsafe {
        for (i, &byte) in text.iter().enumerate() {
            let idx = start_base + i;
            if idx >= 25 * 80 {
                break;
            }

            SKREEN_TEXT[idx] = byte;
            SKREEN_COLOR[idx] = color;

            ITER = (idx + 1) as u32;
        }
    }
}