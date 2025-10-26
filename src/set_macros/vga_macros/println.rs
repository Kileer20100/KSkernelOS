

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        
        $crate::drivers::vga::xd8000::println::println(format_args!($($arg)*));

    };

}