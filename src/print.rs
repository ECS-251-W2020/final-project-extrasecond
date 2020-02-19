use core::fmt;

pub fn _print(args: fmt::Arguments){
    use core::fmt::Write;
    
    crate::bsp::console().write_fmt(args).unwrap();
}

// Crate copied and modified from rust source code.
// Now we have to implement this on our own.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args!($($arg)*));
    })
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}