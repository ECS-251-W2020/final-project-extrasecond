use crate::{arch, arch::Lock, interface};
use core::ops;
use register::{mmio::ReadWrite, register_bitfields, register_structs};

register_bitfields! {
    u32,

    GPFSEL1 [
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100
        ],

        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => GPFSEL0: ReadWrite<u32>),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => GPFSEL2: ReadWrite<u32>),
        (0x0C => GPFSEL3: ReadWrite<u32>),
        (0x10 => GPFSEL4: ReadWrite<u32>),
        (0x14 => GPFSEL5: ReadWrite<u32>),
        (0x18 => _reserved1),
        (0x1c => GPSET0: ReadWrite<u32>),
        (0x20 => GPSET1: ReadWrite<u32>),
        (0x24 => _reserved2),
        (0x28 => GPCLR0: ReadWrite<u32>),
        (0x2c => GPCLR1: ReadWrite<u32>),
        (0x30 => _reserved3),
        (0x34 => GPLEV0: ReadWrite<u32>),
        (0x38 => GPLEV1: ReadWrite<u32>),
        (0x3c => _reserved4),
        (0x94 => GPPUD: ReadWrite<u32>),
        (0x98 => GPPUDCLK0: ReadWrite<u32>),
        (0x9C => GPPUDCLK1: ReadWrite<u32>),
        (0xA0 => @END),
    }
}

struct GPIOInner {
    base_addr: usize,
}

impl ops::Deref for GPIOInner {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl GPIOInner {
    const fn new(base_addr: usize) -> GPIOInner {
        GPIOInner { base_addr }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }
}

// use spin::Mutex;
use interface::sync::Mutex;

pub struct GPIO {
    inner: Lock<GPIOInner>,
}

impl GPIO {
    pub const unsafe fn new(base_addr: usize) -> GPIO {
        GPIO {
            inner: Lock::new(GPIOInner::new(base_addr)),
        }
    }

    pub fn output(&self, pin: u32, value: u32) {
        let mut r = &self.inner;
        r.lock(|inner| {
            if value == 0 {
                let modified = (inner.GPSET0.get() as u32) | (1 << pin);
                inner.GPSET0.set(modified);
            } else {
                let modified = (inner.GPCLR0.get() as u32) | (1 << pin);
                inner.GPCLR0.set(modified);
            }
        })
    }

    pub fn pullupdn(&self, pin: u32, op: u32) {
        let mut r = &self.inner;
        r.lock(|inner| {
            inner.GPPUD.set(op);
            arch::spin_for_cycles(150);

            let modified = (inner.GPPUDCLK0.get() as u32) | (1 << pin);
            inner.GPPUDCLK0.set(modified);
            arch::spin_for_cycles(150);

            inner.GPPUD.set(0);
            inner.GPPUDCLK0.set(0);
        })
    }

    pub fn map_pl011_uart(&self) {
        let mut r = &self.inner;
        r.lock(|inner| {
            inner
                .GPFSEL1
                .modify(GPFSEL1::FSEL14::AltFunc0 + GPFSEL1::FSEL15::AltFunc0);

            inner.GPPUD.set(0);
            arch::spin_for_cycles(150);
            
            let modified = (inner.GPPUDCLK0.get() as u32) | (1 << 14) | (1 << 15);
            inner.GPPUDCLK0.set(modified);

            arch::spin_for_cycles(150);

            inner.GPPUDCLK0.set(0);
        })
    }
}

impl interface::driver::DeviceDriver for GPIO {
    fn compatible(&self) -> &str {
        "GPIO"
    }
}
