pub mod console {
    use core::fmt;

    pub trait Write {
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }
}

pub mod sync {
    pub trait Mutex {
        type Data;

        fn lock<R, F>(&mut self, f: F) -> R
        where F: FnOnce(&mut Self::Data) -> R ;
    }
}