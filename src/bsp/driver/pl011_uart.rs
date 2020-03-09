use crate::{arch::Lock, interface};
use asm::nop;
use core::{fmt, ops};
use cortex_a::asm;
use register::{mmio::*, register_bitfields, register_structs};

register_bitfields! {
    u32,

    FR [
        TXFE OFFSET(7) NUMBITS(1) [],
        TXFF OFFSET(5) NUMBITS(1) [],
        RXFE OFFSET(4) NUMBITS(1) []
    ],

    IBRD [
        IBRD OFFSET(0) NUMBITS(16) []
    ],

    FBRD [
        FBRD OFFSET(0) NUMBITS(6) []
    ],

    LCRH [
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit     = 0b00,
            SixBit      = 0b01,
            SevenBit    = 0b10,
            EightBit    = 0b11
        ],

        FEN OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1
        ]
    ],

    CR [
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        UARTEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    ICR [
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => LCRH: WriteOnly<u32, LCRH::Register>),
        (0x30 => CR: WriteOnly<u32, CR::Register>),
        (0x34 => _reserved3),
        (0x44 => ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

pub struct PL011UartInner {
    base_addr: usize,
    chars_written: usize,
    chars_read: usize,
}

impl ops::Deref for PL011UartInner {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl PL011UartInner {
    pub const unsafe fn new(base_addr: usize) -> PL011UartInner {
        PL011UartInner {
            base_addr,
            chars_written: 0,
            chars_read: 0,
        }
    }

    pub fn init(&self) {
        self.CR.set(0);
        self.ICR.write(ICR::ALL::CLEAR);
        self.IBRD.write(IBRD::IBRD.val(13));
        self.FBRD.write(FBRD::FBRD.val(2));
        self.LCRH
            .write(LCRH::WLEN::EightBit + LCRH::FEN::FifosEnabled);
        self.CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.write_char('\r');
        }
        while self.FR.matches_all(FR::TXFF::SET) {
            nop();
        }

        self.DR.set(c as u32);
        self.chars_written += 1;
    }
}

impl fmt::Write for PL011UartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}

use crate::interface::sync::Mutex;
// use spin::Mutex;

pub struct PL011Uart {
    inner: Lock<PL011UartInner>,
}

impl PL011Uart {
    pub const unsafe fn new(base_addr: usize) -> PL011Uart {
        PL011Uart {
            inner: Lock::new(PL011UartInner::new(base_addr)),
        }
    }
}

impl interface::driver::DeviceDriver for PL011Uart {
    fn compatible(&self) -> &str {
        "PL011Uart"
    }

    fn init(&self) -> interface::driver::Result {
        let r = &self.inner;
        r.mutex_use(|inner| inner.init());

        Ok(())
    }
}

impl interface::console::Write for PL011Uart {
    fn write_char(&self, c: char) {
        let r = &self.inner;
        r.mutex_use(|inner| inner.write_char(c));
    }

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        let r = &self.inner;
        r.mutex_use(|inner| fmt::Write::write_fmt(inner, args))
    }

    fn flush(&self) {
        let r = &self.inner;
        // Spin until TX FIFO empty is set.
        r.mutex_use(|inner| {
            while !inner.FR.matches_all(FR::TXFE::SET) {
                nop();
            }
        });
    }
}

impl interface::console::Read for PL011Uart {
    fn read_char(&self) -> char {
        let r = &self.inner;
        r.mutex_use(|inner| {
            // Spin while RX FIFO empty is set.
            while inner.FR.matches_all(FR::RXFE::SET) {
                nop();
            }

            // Read one character.
            let mut ret = inner.DR.get() as u8 as char;

            // Convert carrige return to newline.
            if ret == '\r' {
                ret = '\n'
            }

            // Update statistics.
            inner.chars_read += 1;

            ret
        })
    }

    fn clear(&self) {
        let r = &self.inner;
        r.mutex_use(|inner| {
            // Read from the RX FIFO until it is indicating empty.
            while !inner.FR.matches_all(FR::RXFE::SET) {
                inner.DR.get();
            }
        })
    }
}

impl interface::console::Statistics for PL011Uart {
    fn chars_written(&self) -> usize {
        let r = &self.inner;
        r.mutex_use(|inner| inner.chars_written)
    }

    fn chars_read(&self) -> usize {
        let r = &self.inner;
        r.mutex_use(|inner| inner.chars_read)
    }
}

pub use PL011UartInner as PanicUart;
