use crate::interface::pwm::All as PWMAll;
use crate::{arch, arch::Mutex, bsp, interface, interface::time::Timer};
use core::{ops, time::Duration};
use register::mmio::{ReadOnly, ReadWrite, WriteOnly};
use register::{register_bitfields, register_structs};

register_bitfields! {
    u32,

    GPFSEL0 [
        FSEL2 OFFSET(6) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL1 OFFSET(3) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL0 OFFSET(0) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ]
    ],

    GPFSEL1 [
        FSEL19 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc5 = 0b010
        ],

        FSEL18 OFFSET(24) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc5 = 0b010
        ],

        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100
        ],

        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100
        ],

        FSEL13 OFFSET(9) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PWM Channel 0
        ],

        FSEL12 OFFSET(6) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PWM Channel 0
        ]
    ],

    GPCLR0 [
        CLR2 OFFSET(2) NUMBITS(1) [
            NoEffect = 0,
            Clear = 1
        ],

        CLR1 OFFSET(1) NUMBITS(1) [
            NoEffect = 0,
            Clear = 1
        ],

        CLR0 OFFSET(0) NUMBITS(1) [
            NoEffect = 0,
            Clear = 1
        ]
    ],

    GPLEV0 [
        LEV3 OFFSET(3) NUMBITS(1) [],
        LEV2 OFFSET(2) NUMBITS(1) [],
        LEV1 OFFSET(1) NUMBITS(1) [],
        LEV0 OFFSET(0) NUMBITS(1) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => GPFSEL0: ReadWrite<u32, GPFSEL0::Register>),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => GPFSEL2: ReadWrite<u32>),
        (0x0C => GPFSEL3: ReadWrite<u32>),
        (0x10 => GPFSEL4: ReadWrite<u32>),
        (0x14 => GPFSEL5: ReadWrite<u32>),
        (0x18 => _reserved1),
        (0x1c => GPSET0: WriteOnly<u32>),
        (0x20 => GPSET1: WriteOnly<u32>),
        (0x24 => _reserved2),
        (0x28 => GPCLR0: WriteOnly<u32>),
        (0x2c => GPCLR1: WriteOnly<u32>),
        (0x30 => _reserved3),
        (0x34 => GPLEV0: ReadOnly<u32>),
        (0x38 => GPLEV1: ReadOnly<u32>),
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

pub struct GPIO {
    inner: Mutex<GPIOInner>,
}

impl GPIO {
    pub const unsafe fn new(base_addr: usize) -> GPIO {
        GPIO {
            inner: Mutex::new(GPIOInner::new(base_addr)),
        }
    }

    pub fn map_pl011_uart(&self) {
        let inner = &self.inner.lock();
        inner
            .GPFSEL1
            .modify(GPFSEL1::FSEL14::AltFunc0 + GPFSEL1::FSEL15::AltFunc0);

        inner.GPPUD.set(0);
        arch::spin_for_cycles(150);

        let modified = (inner.GPPUDCLK0.get() as u32) | (1 << 14) | (1 << 15);
        inner.GPPUDCLK0.set(modified);

        arch::spin_for_cycles(150);

        inner.GPPUDCLK0.set(0);
    }
}

use interface::gpio::{Dir, Pud};

impl interface::gpio::Set for GPIO {
    fn pullupdn(&self, pin: u32, pud: Pud) {
        let pull = match pud {
            Pud::PudOff => 0,
            Pud::PudUp => 1,
            Pud::PudDown => 2,
        };

        let inner = &self.inner.lock();
        inner.GPPUD.set(pull);
        arch::spin_for_cycles(150);

        let modified = (inner.GPPUDCLK0.get() as u32) | (1 << pin);
        inner.GPPUDCLK0.set(modified);
        arch::spin_for_cycles(150);

        inner.GPPUD.set(0);
        inner.GPPUDCLK0.set(0);
    }

    fn setup(&self, pin: u32, direction: Dir, pud: Pud) {
        self.pullupdn(pin, pud);

        let inner = &self.inner.lock();
        let d = match direction {
            Dir::Input => 0,
            Dir::Output => 1,
        };
        match pin {
            0..10 => {
                inner.GPFSEL0.set(
                    (inner.GPFSEL0.get() & 0xFFFFFFF8_u32.rotate_left(pin * 3)) | (d << (pin * 3)),
                );
            }
            10..20 => {
                inner.GPFSEL1.set(
                    (inner.GPFSEL1.get() & 0xFFFFFFF8_u32.rotate_left((pin - 10) * 3))
                        | (d << ((pin - 10) * 3)),
                );
            }
            20..28 => {
                inner.GPFSEL2.set(
                    (inner.GPFSEL2.get() & 0xFFFFFFF8_u32.rotate_left((pin - 20) * 3))
                        | (d << ((pin - 20) * 3)),
                );
            }
            _ => {
                arch::spin_for_cycles(1);
            }
        };
        inner.GPCLR0.set(1 << pin);
    }

    fn setup_pwm(&self, pin: u32) {
        let inner = &self.inner.lock();
        if pin == 12 {
            inner.GPFSEL1.modify(GPFSEL1::FSEL12::AltFunc0);
        } else if pin == 13 {
            inner.GPFSEL1.modify(GPFSEL1::FSEL13::AltFunc0);
        } else if pin == 18 {
            inner.GPFSEL1.modify(GPFSEL1::FSEL18::AltFunc5);
        } else if pin == 19 {
            inner.GPFSEL1.modify(GPFSEL1::FSEL19::AltFunc5);
        } else {
            return;
        }
        arch::timer().spin_for(Duration::from_secs_f32(0.11));
        bsp::pwm().set_mode(1);
        bsp::pwm().set_range(1024);
        bsp::pwm().set_clock(32);
    }

    fn cleanup(&self) {
        let inner = &self.inner.lock();
        inner.GPCLR0.set(0xFFFFFFFF_u32);
    }
}
impl interface::gpio::Output for GPIO {
    fn output(&self, pin: u32, value: u32) {
        let inner = &self.inner.lock();
        if value == 0 {
            inner.GPCLR0.set(1 << pin);
        } else {
            inner.GPSET0.set(1 << pin);
        }
    }
}

impl interface::gpio::Input for GPIO {
    fn input(&self, pin: u32) -> u32 {
        let inner = &self.inner.lock();
        ((inner.GPLEV0.get() as u32) >> pin) & 1
    }
}

impl interface::driver::DeviceDriver for GPIO {
    fn compatible(&self) -> &str {
        "GPIO"
    }
}
