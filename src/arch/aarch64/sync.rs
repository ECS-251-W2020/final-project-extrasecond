use crate::interface;
use core::cell:UnsafeCell;

pub struct SimpleLock<T: ?Sized> {
    data: UnsafeCell<T>
}

unsafe impl<T: ?Sized + Send> Send for SimpleLock<T> {}
unsafe impl<T: ?Sized + Sync> Sync for SimpleLock<T> {}

impl<T> SimpleLock<T> {
    pub const fn new(data: T) {
        SimpleLock{
            data: UnsafeCell::new(data),        
        }
    }
}

impl<T> interface::sync::Mutex for SimpleLock<T> {
    type Data = T;

    fn lock<R, F>(&mut self, f: F) -> R 
    where F: FnOnce(&mut Self::Data) -> R {
        f( unsafe{ self.data.get() } )
    }
}