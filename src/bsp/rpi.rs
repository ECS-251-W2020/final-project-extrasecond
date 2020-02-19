use crate::interface;
use core::fmt;

pub const BOOT_CORE_ID: u64 = 0;
pub const BOOT_CORE_STACK_START: u64 = 0x80_000;

struct QEMUOutput;

impl interface::console::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result{
        for c in s.chars(){
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }
        Ok(())
    }
}

pub fn console() -> impl interface::console::Write {
    QEMUOutput {}
}