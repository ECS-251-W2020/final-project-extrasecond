use crate::{arch, arch::Mutex, interface::time::Timer};
use core::time::Duration;
use core::ops;
use register::mmio::ReadWrite;
use register::register_structs;

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x0000 => _reserved1),
        (0x1098 => CM_PCM_CTRL: ReadWrite<u32>),
        (0x109C => CM_PCM_DIV: ReadWrite<u32>),
        (0x1100 => @END),
    }
}

struct ClockInner {
    base_addr: usize,
}

impl ops::Deref for ClockInner {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl ClockInner {
    const fn new(base_addr: usize) -> ClockInner {
        ClockInner { base_addr }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }
}

pub struct Clock {
    inner: Mutex<ClockInner>,
}

impl Clock {
    pub const unsafe fn new(base_addr: usize) -> Clock {
        Clock {
            inner: Mutex::new(ClockInner::new(base_addr)),
        }
    }

    pub fn init(&self, divisor: u32) {
        let inner = &self.inner.lock();
        let bcm_password = 0x5A00_0000;

        inner.CM_PCM_CTRL.set(bcm_password | 0x01);
        arch::timer().spin_for(Duration::from_secs_f32(0.11));

        while (inner.CM_PCM_CTRL.get() & 0x80) != 0 {
            arch::timer().spin_for(Duration::from_secs_f32(0.001));
        }

        inner.CM_PCM_DIV.set(bcm_password | (divisor << 12));
        inner.CM_PCM_CTRL.set(bcm_password | 0x11);
    }
}