pub mod console {
    use core::fmt;

    pub trait Write {
        fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result;

        fn write_char(&mut self, c: char);
    }

    pub trait Read {
        fn read_char(&mut self) -> char {
            ' '
        }
    }

    pub trait Statistics {
        fn chars_written(&mut self) -> usize {
            0
        }

        fn chars_read(&mut self) -> usize {
            0
        }
    }

    pub trait All = Write + Read + Statistics;
}

pub mod driver {
    pub type Result = core::result::Result<(), ()>;

    pub trait DeviceDriver {
        fn compatible(&self) -> &str;

        fn init(&mut self) -> Result {
            Ok(())
        }
    }
}

pub mod sync {
    pub trait Mutex {
        type Data;

        fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    }
}

pub mod time {
    use core::time::Duration;

    /// Timer functions.
    pub trait Timer {
        /// The timer's resolution.
        fn resolution(&self) -> Duration;

        /// The uptime since power-on of the device.
        ///
        /// This includes time consumed by firmware and bootloaders.
        fn uptime(&self) -> Duration;

        /// Spin for a given duration.
        fn spin_for(&self, duration: Duration);
    }
}
