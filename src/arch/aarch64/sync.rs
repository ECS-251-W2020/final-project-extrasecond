// mod cmpset;

use crate::interface;
use core::cell::UnsafeCell;
// use crate::arch::sync::cmpset::compare_and_set;

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

impl<T> interface::sync::Mutex for Lock<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        f(unsafe { &mut *self.data.get() })
    }
}

/*
#[derive(Debug)]
pub struct ArmLock<T: ?Sized> {
    lock: bool,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for ArmLock<T> {}
unsafe impl<T: ?Sized + Send> Sync for ArmLock<T> {}

impl<T> ArmLock<T> {
    pub const fn new(data: T) -> ArmLock<T> {
        ArmLock {
            lock: false,
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> ArmLock<T> {
    fn acquire_lock(&mut self) {
        while compare_and_set(&mut self.lock, false, true) == false {}
    }
    fn release_lock(&mut self) {
        self.lock = false;
    }
}

impl<T> interface::sync::Mutex for ArmLock<T>{
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R{
        self.acquire_lock();
        let ret = f(unsafe { &mut *self.data.get() });
        self.release_lock();
        ret
    }
}*/
