impl<T> crate::interface::sync::Mutex for spin::Mutex<T> {
    type Data = T;
    fn mutex_use<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let mut data = spin::Mutex::lock(self);
        f(&mut data)
    }
}

pub use spin::Mutex as Lock;