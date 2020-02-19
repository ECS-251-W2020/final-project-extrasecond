pub mod console {
    pub use core::fmt::Write;

}

pub mod sync {
    pub trait Mutex {
        type Data;

        fn lock<R, F>(&mut self, f: F) -> R
        where F: FnOnce(&mut Self::Data) -> R ;
    }
}