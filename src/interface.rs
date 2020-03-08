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

    pub trait ConsoleAll = Write + Read + Statistics;
}


pub mod gpio {

    pub enum Pud {
        PudOff, 
        PudUp, 
        PudDown
    }

    pub trait Set {
        fn pullupdn(&self, pin: u32, pud: Pud);

        fn setup(&self, pin: u32, direction: u32, pud: Pud);

        fn cleanup(&self);
    }

    pub trait Output {
        fn output(&self, pin: u32, value: u32);
    }

    pub trait Input {
        fn input(&self, pin: u32) -> u32;
    }

    pub trait GPIOAll = Set + Output + Input;
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

pub mod sync {
    pub trait Mutex {
        type Data;

        fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    }
}
