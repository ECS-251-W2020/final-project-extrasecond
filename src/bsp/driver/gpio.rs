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
    ],

    GPPUDCLK0 [
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
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
        (0x94 => GPPUD: ReadWrite<u32>),
        (0x98 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>),
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
use crate::interface::sync::Mutex;

pub struct GPIO {
    inner: Lock<GPIOInner>,
}

impl GPIO {
    pub const unsafe fn new(base_addr: usize) -> GPIO {
        GPIO {
            inner: Lock::new(GPIOInner::new(base_addr)),
        }
    }

    pub fn map_pl011_uart(&mut self) {
        let r = &mut self.inner;
        r.lock(|inner| {
            inner
                .GPFSEL1
                .modify(GPFSEL1::FSEL14::AltFunc0 + GPFSEL1::FSEL15::AltFunc0);

            inner.GPPUD.set(0);
            arch::spin_for_cycles(150);

            inner
                .GPPUDCLK0
                .write(GPPUDCLK0::PUDCLK14::AssertClock + GPPUDCLK0::PUDCLK15::AssertClock);
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
