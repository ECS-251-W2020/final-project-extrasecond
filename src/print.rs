use core::fmt;

pub fn _print(args: fmt::Arguments) {
    use crate::interface::console::Write;
    crate::bsp::console().write_fmt(args).unwrap();
}

// Crate copied and modified from rust source code.
// Now we have to implement this on our own.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args_nl!($($arg)*));
    })
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Prints an info, with newline.
#[macro_export]
macro_rules! info {
    ($string:expr) => ({
        info!("{}", $string);
    });
    ($format_string:expr, $($arg:tt)*) => ({
        #[allow(unused_imports)]
        use crate::interface::time::Timer;

        let timestamp = $crate::arch::timer().uptime();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::print::_print(format_args_nl!(
            concat!("[Info {:>3}.{:03}{:03}] ", $format_string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            timestamp_subsec_us % 1_000,
            $($arg)*
        ));
    })
}

#[macro_export]
macro_rules! warn {
    ($string:expr) => ({
        warn!("{}", $string);
    });
    ($format_string:expr, $($arg:tt)*) => ({
        #[allow(unused_imports)]
        use crate::interface::time::Timer;

        let timestamp = $crate::arch::timer().uptime();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::print::_print(format_args_nl!(
            concat!("[Warn {:>3}.{:03}{:03}] ", $format_string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            timestamp_subsec_us % 1_000,
            $($arg)*
        ));
    })
}
