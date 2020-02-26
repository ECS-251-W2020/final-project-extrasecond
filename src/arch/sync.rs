use crate::interface;
use core::cell::UnsafeCell;

pub struct Lock<T: ?Sized> {
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for Lock<T> {}
unsafe impl<T: ?Sized + Send> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub const fn new(data: T) -> Lock<T> {
        Lock {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> interface::sync::Mutex for &Lock<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        f(unsafe { &mut *self.data.get() })
    }
}
