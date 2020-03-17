pub mod console {
    use core::fmt;

    pub trait Write {
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

        fn write_char(&self, c: char);

        /// Block execution until the last character has been physically put on the TX wire
        /// (draining TX buffers/FIFOs, if any).
        fn flush(&self);
    }

    pub trait Read {
        fn read_char(&self) -> char {
            ' '
        }
        fn clear(&self);
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

pub mod pwm {
    #[allow(dead_code)]
    pub trait Set {
        fn set_mode(&self, mode: u32);

        fn set_range(&self, range: u32);

        fn set_clock(&self, divisor: u32);
    }

    pub trait Output {
        fn write(&self, pin: u32, value: u32);
    }

    pub trait All = Set + Output;
}

pub mod gpio {
    #[allow(dead_code)]
    pub enum Pud {
        PudOff,
        PudUp,
        PudDown,
    }

    pub enum Dir {
        Input,
        Output,
    }

    pub trait Set {
        fn pullupdn(&self, pin: u32, pud: Pud);

        fn setup(&self, pin: u32, direction: Dir, pud: Pud);

        fn setup_pwm(&self, pin: u32);

        fn cleanup(&self);
    }

    pub trait Output {
        fn output(&self, pin: u32, value: u32);
    }

    pub trait Input {
        fn input(&self, pin: u32) -> u32;
    }

    pub trait All = Set + Output + Input;
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

pub mod mm {
    pub trait MMU {
        unsafe fn init(&self) -> Result<(), &'static str>;
    }
}
