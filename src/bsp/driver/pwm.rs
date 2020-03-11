use crate::{arch, arch::Mutex, interface, interface::time::Timer};
use core::time::Duration;
use core::ops;
use register::mmio::ReadWrite;
use register::{register_bitfields, register_structs};
use crate::bsp::driver::clock::Clock;

register_bitfields! {
    u32,

    CTL [
        MSEN2 OFFSET(15) NUMBITS(1) [
            PWMAlgorithm = 0,
            MSTransmission = 1
        ],

        USEF2 OFFSET(13) NUMBITS(1) [
            Transmitted = 0,
            Fifo = 1
        ],

        POLA2 OFFSET(12) NUMBITS(1) [
            LowHigh = 0, // 0 = low, 1 = high
            HighLow = 1  // 0 = high, 1 = low
        ],

        SBIT2 OFFSET(11) NUMBITS(1) [],

        RPTL2 OFFSET(10) NUMBITS(1) [
            Interrupt = 0, // Transmission interrupts when FIFO is empty
            Repeat = 1     // Last data in FIFO is transmitted repetedly until FIFO is not empty
        ],

        MODE2 OFFSET(9) NUMBITS(1) [
            PWM = 0,
            Serialiser = 1
        ],

        PWEN2 OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        MSEN1 OFFSET(7) NUMBITS(1) [
            PWMAlgorithm = 0,
            MSTransmission = 1
        ],

        CLRF1 OFFSET(6) NUMBITS(1) [
            NoEffect = 0,
            ClearFIFO = 1
        ],

        USEF1 OFFSET(5) NUMBITS(1) [
            Transmitted = 0,
            Fifo = 1
        ],

        POLA1 OFFSET(4) NUMBITS(1) [
            LowHigh = 0, // 0 = low, 1 = high
            HighLow = 1  // 0 = high, 1 = low
        ],

        SBIT1 OFFSET(3) NUMBITS(1) [],

        RPTL1 OFFSET(2) NUMBITS(1) [
            Interrupt = 0, // Transmission interrupts when FIFO is empty
            Repeat = 1     // Last data in FIFO is transmitted repetedly until FIFO is not empty
        ],

        MODE1 OFFSET(1) NUMBITS(1) [
            PWM = 0,
            Serialiser = 1
        ],

        PWEN1 OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    STA [
        STA4 OFFSET(12) NUMBITS(1) [],
        STA3 OFFSET(11) NUMBITS(1) [],
        STA2 OFFSET(10) NUMBITS(1) [],
        STA1 OFFSET(9) NUMBITS(1) [],

        BERR OFFSET(8) NUMBITS(1) [],

        GAPO4 OFFSET(7) NUMBITS(1) [],
        GAPO3 OFFSET(6) NUMBITS(1) [],
        GAPO2 OFFSET(5) NUMBITS(1) [],
        GAPO1 OFFSET(4) NUMBITS(1) [],

        RERR1 OFFSET(3) NUMBITS(1) [],
        WERR1 OFFSET(2) NUMBITS(1) [],

        EMPT1 OFFSET(1) NUMBITS(1) [],
        FULL1 OFFSET(0) NUMBITS(1) []
    ],

    DMAC [
        ENAB OFFSET(31) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        PANIC OFFSET(8) NUMBITS(8) [],
        DREQ OFFSET(0) NUMBITS(8) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => CTL: ReadWrite<u32, CTL::Register>),
        (0x04 => STA: ReadWrite<u32, STA::Register>),
        (0x08 => DMAC: ReadWrite<u32, DMAC::Register>),
        (0x0C => _reserved1),
        (0x10 => RNG1: ReadWrite<u32>),
        (0x14 => DAT1: ReadWrite<u32>),
        (0x18 => FIF1: ReadWrite<u32>),
        (0x1C => _reserved2),
        (0x20 => RNG2: ReadWrite<u32>),
        (0x24 => DAT2: ReadWrite<u32>),
        (0x28 => @END),
    }
}


struct PWMInner {
    base_addr: usize,
}

impl ops::Deref for PWMInner {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl PWMInner {
    const fn new(base_addr: usize) -> PWMInner {
        PWMInner { base_addr }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }
}


pub struct PWM {
    inner: Mutex<PWMInner>,
    clock: Clock,
    gpio_to_pwm_port: [u32; 28],
}

impl PWM {
    pub const unsafe fn new(base_addr: usize, clock_base_addr: usize) -> PWM {
        PWM {
            inner: Mutex::new(PWMInner::new(base_addr)),
            clock: Clock::new(clock_base_addr),
            gpio_to_pwm_port: [
                0,          0,          0,          0,          0,          0,          0,          0,	//  0 ->  7
                0,          0,          0,          0,          1,          2,          0,          0, 	//  8 -> 15
                0,          0,          1,          2,          0,          0,          0,          0, 	// 16 -> 23
                0,          0,          0,          0	                                                // 24 -> 28
            ]
        }
    }
}

impl interface::pwm::Set for PWM {

    fn set_mode(&self, mode: u32) {
        let inner = &self.inner.lock();
        if mode == 0 {
            inner.CTL.modify(CTL::PWEN1::Enabled + CTL::PWEN2::Enabled + CTL::MSEN1::MSTransmission + CTL::MSEN2::MSTransmission);
        } else {
            inner.CTL.modify(CTL::PWEN1::Enabled + CTL::PWEN2::Enabled);
        }
    }

    fn set_range(&self, range: u32) {
        let inner = &self.inner.lock();
        inner.RNG1.set(range);
        arch::timer().spin_for(Duration::from_secs_f32(0.01));

        inner.RNG2.set(range);
        arch::timer().spin_for(Duration::from_secs_f32(0.01));
    }

    fn set_clock(&self, divisor: u32) {
        let inner = &self.inner.lock();
        let pwm_control = inner.CTL.get() as u32;
        inner.CTL.set(0);
        
        &self.clock.init(divisor & 0x0FFF);
        inner.CTL.set(pwm_control);
    }
}

impl interface::pwm::Output for PWM {
    fn write(&self, pin: u32, value: u32) {
        let inner = &self.inner.lock();
        match &self.gpio_to_pwm_port[pin as usize] {
            0 => arch::spin_for_cycles(1),
            1 => inner.DAT1.set(value),
            2 => inner.DAT2.set(value),
            _ => arch::spin_for_cycles(1),
        };
    }
}

