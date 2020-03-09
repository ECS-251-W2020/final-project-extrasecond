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

            let modified = (inner.GPPUDCLK0.get() as u32) | (1 << 14) | (1 << 15);
            inner.GPPUDCLK0.set(modified);

            arch::spin_for_cycles(150);

            inner.GPPUDCLK0.set(0);
        });
    }
}

use interface::gpio::Pud;

impl interface::gpio::Set for GPIO {
    fn pullupdn(&mut self, pin: u32, pud: Pud) {
        let pull = match pud {
            Pud::PudOff => 0,
            Pud::PudUp => 1,
            Pud::PudDown => 2,
        };

        let r = &mut self.inner;
        r.lock(|inner| {
            inner.GPPUD.set(pull);
            arch::spin_for_cycles(150);

            let modified = (inner.GPPUDCLK0.get() as u32) | (1 << pin);
            inner.GPPUDCLK0.set(modified);
            arch::spin_for_cycles(150);

            inner.GPPUD.set(0);
            inner.GPPUDCLK0.set(0);
        });
    }

    fn setup(&mut self, pin: u32, direction: u32, pud: interface::gpio::Pud) {
        self.pullupdn(pin, pud);

        let r = &mut self.inner;
        r.lock(|inner| match pin {
            0..10 => {
                let modified = (inner.GPFSEL0.get() as u32) | (direction << (pin * 3));
                inner.GPFSEL0.set(modified);
            }
            10..20 => {
                let modified = (inner.GPFSEL1.get() as u32) | (direction << ((pin - 10) * 3));
                inner.GPFSEL1.set(modified);
            }
            20..29 => {
                let modified = (inner.GPFSEL2.get() as u32) | (direction << ((pin - 20) * 3));
                inner.GPFSEL2.set(modified);
            }
            _ => {
                arch::spin_for_cycles(1);
            }
        });
    }

    fn cleanup(&mut self) {
        arch::spin_for_cycles(1);
    }
}
impl interface::gpio::Output for GPIO {
    fn output(&mut self, pin: u32, value: u32) {
        let r = &mut self.inner;
        r.lock(|inner| {
            if value == 0 {
                let modified = (inner.GPSET0.get() as u32) | (1 << pin);
                inner.GPSET0.set(modified);
            } else {
                let modified = (inner.GPCLR0.get() as u32) | (1 << pin);
                inner.GPCLR0.set(modified);
            }
        });
    }
}

impl interface::gpio::Input for GPIO {
    fn input(&mut self, pin: u32) -> u32 {
        let r = &mut self.inner;
        r.lock(|inner| (inner.GPLEV1.get() as u32 >> pin) | 1)
    }
}

impl interface::driver::DeviceDriver for GPIO {
    fn compatible(&self) -> &str {
        "GPIO"
    }
}
