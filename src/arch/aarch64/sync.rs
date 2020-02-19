use crate::interface;
use core::cell:UnsafeCell;

pub struct SimpleLock<T: ?Sized> {
    data: UnsafeCell<T>
}

unsafe impl<T: ?Sized + Send> Send for SimpleLock<T> {}
unsafe impl<T: ?Sized + Sync> Sync for SimpleLock<T> {}