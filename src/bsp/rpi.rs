use crate::interface;
use core::fmt;
use core::fmt::Write;

pub const BOOT_CORE_ID: u64 = 0;
pub const BOOT_CORE_STACK_START: u64 = 0x80_000;

struct QEMUOutputInner {
    chars_written: usize,
}

impl QEMUOutputInner {
    const fn new() -> Self {
        QEMUOutputInner { chars_written: 0 }
    }
}

impl core::fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }
        self.chars_written += s.len();
        Ok(())
    }
}

use spin::Mutex;

struct QEMUOutput {
    inner: Mutex<QEMUOutputInner>,
}
impl QEMUOutput {
    const fn new() -> Self {
        QEMUOutput {
            inner: Mutex::new(QEMUOutputInner::new()),
        }
    }
}

impl interface::console::Write for QEMUOutput {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        self.inner.lock().write_fmt(args)
    }
}

static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

pub fn console() -> &'static impl interface::console::Write {
    &QEMU_OUTPUT
}
