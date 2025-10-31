///----------------------------------------///
///*****************VGA********************///
/// VGA println implementation for XD8000  ///
///****************************************///
///----------------------------------------///

// Using Color from the color module
use crate::drivers::vga::xd8000::color::{Color};

// Using core::fmt::Write for implementing the Write trait
use core::fmt::Write;

// CODE IMPLEMENTATION
// Static mutable buffers for text and color attributes
static mut SKREEN_TEXT: [u8; 25*80] = [0; 25*80];
static mut SKREEN_COLOR: [u8; 25*80] = [0; 25*80];


// Global mutable variables to track iteration and row writing
static mut ITER: u32 = 0;
static mut ROW_WRITE: u32 = 0;

// Global mutable variable for write color
static mut WRITE_COLOR: u8 = Color::WHITE;

//*-END MACROS println!();-*/
pub fn println(agrs: core::fmt::Arguments) {
    let mut printer = VGAPrinter;
    unsafe {
        WRITE_COLOR = Color::WHITE;
    }
    core::fmt::Write::write_fmt(&mut printer, agrs).unwrap();
}

//*-END MACROS println_warn();-*/
pub fn println_warn(agrs: core::fmt::Arguments) {
    let mut printer = VGAPrinter;

    unsafe {
        WRITE_COLOR = Color::warning();
    }

    core::fmt::Write::write_fmt(&mut printer, agrs).unwrap();
}

//*-END MACROS println_warn();-*/
pub fn println_error(agrs: core::fmt::Arguments) {
    let mut printer = VGAPrinter;

    unsafe {
        WRITE_COLOR = Color::error();
    }

    core::fmt::Write::write_fmt(&mut printer, agrs).unwrap();
}


struct VGAPrinter;

// Implementing the Write trait for VGAPrinter
impl Write for VGAPrinter {
    fn write_str(&mut self, message: &str) -> core::fmt::Result {
            //let text = message.bytes().enumerate();
            //call the buffer function 
            bufer_vga(message);

            //write the buffered data to VGA memory
            let base = 0xb8000 as *mut u8;

            //- - - - - - - - - - - - - - - - - - - - - - - - -//
            //get the global iteration and calculate the row
            let global_iteration = unsafe { ITER };
            let row = global_iteration / 80;
            let vga_buffer = unsafe {
                if global_iteration == 0 {
                    0xb8000 as *mut u8
                } else {
                    base.add((row as usize) * 0xa0)
                }
            };
            //- - - - - - - - - - - - - - - - - - - - - - - - -//
            //write the current row from the buffers to VGA memory
            //- - - - - - - - - - - - - - - - - - - - - - - - -//
            //get the slice for the current row
            unsafe {
            //get the slice indices for the current row
            let (start, stop) = slise_buffer_row();
            //get the slices from the text and color buffers
            let slise_text_buffer = &SKREEN_TEXT[start..stop];
            let slise_color_buffer = &SKREEN_COLOR[start..stop];
            //write the slices to VGA memory

                for i in 0..80{
                    //write character byte
                    *vga_buffer.offset(i as isize * 2) = slise_text_buffer[i];
                    //write color byte
                    *vga_buffer.offset(i as isize * 2 + 1) = slise_color_buffer[i];
                    
                }
            Ok(())
        }
        //- - - - - - - - - - - - - - - - - - - - - - - - -//
    }
}

//*-Function to buffer VGA output-*/
fn bufer_vga(message: &str) {
    //convert message to bytes
    let text = message.as_bytes();

    //get the global write color
    let global_color = unsafe { WRITE_COLOR };

    //call global iteration function
    global_iteration_();

    //get the start base for writing
    let start_base = unsafe { (ROW_WRITE as usize) * 80 };

    //write the text and color to the buffers
    unsafe {
        for (i, &byte) in text.iter().enumerate() {
            let idx = start_base + i;
            if idx >= 25 * 80 {
                break;
            }

            SKREEN_TEXT[idx] = byte;
            SKREEN_COLOR[idx] = global_color as u8;

            ITER = (idx + 1) as u32;
        }
    }
}
//*-Function to manage global iteration and row writing-*/
fn global_iteration_() {
    //get the global iteration
    let global_iteration = unsafe { ITER };

    //calculate the current row
    let row = global_iteration / 80;
    //update the ROW_WRITE based on the current row
    unsafe {
        let row_write: u32;

        if row == 0 {
            //check if the first two characters are non-zero
            let logick_slise = &SKREEN_TEXT[0..2];

            if logick_slise[0] != 0 && logick_slise[1] != 0 {
                //move to the next row
                row_write = 1;
            } else {
                //reset to the first row
                row_write = 0;
            } 
        } 
        else if row > 0 && row < 24 {
            //move to the next row
            row_write = row + 1;
        } 
        else {
            //clear the buffer and reset to the first row
            clear_buffer();
            row_write = 0;
        }
        //update the ROW_WRITE variable
        ROW_WRITE = row_write;
    }
}
//*-Function to clear the text and color buffers-*/
fn clear_buffer() {
    unsafe {
        //clear the SKREEN_TEXT and SKREEN_COLOR buffers
        for i in 0..(25 * 80) {
            SKREEN_TEXT[i] = 0;
            SKREEN_COLOR[i] = 0;
        }

        //clear the VGA memory
        let base = 0xb8000 as *mut u8;
        //set each character to space and color to black
        for i in 0..(25 * 80) {
            *base.add(i * 2) = b' ';
            *base.add(i * 2 + 1) = Color::BLACK;
        }
    }
}
//*-Function to get the slice indices for the current row in the buffers-*/
fn slise_buffer_row() -> (usize, usize) {
    //get the global iteration
    let global_iteration = unsafe { ITER };
    //calculate the current row
    let row = global_iteration / 80;
    //return the start and stop indices for the current row
    ((80 * row) as usize, (80 * (row + 1)) as usize)
}


