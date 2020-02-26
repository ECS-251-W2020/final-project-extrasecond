pub mod console {
    use core::fmt;

    pub trait Write {
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

        fn write_char(&self, c: char);
    }

    pub trait Read {
        fn read_char(&self) -> char {
            ' '
        }
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }

        fn chars_read(&self) -> usize {
            0
        }
    }

    pub trait All = Write + Read + Statistics;
}

pub mod driver {
    pub type Result = core::result::Result<(), ()>;

    pub trait DeviceDriver {
        fn compatible(&self) -> &str;

        fn init(&self) -> Result {
            Ok(())
        }
    }
}
