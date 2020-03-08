pub mod sync;
mod time;

use crate::{bsp, interface};
use cortex_a::{asm, regs::*};

static TIMER: time::Timer = time::Timer;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    const CORE_MASK: u64 = 0x3; // The last two bits for 4 cores
    
    if bsp::BOOT_CORE_ID == MPIDR_EL1.get() & CORE_MASK {
        SP.set(bsp::BOOT_CORE_STACK_START);
        crate::runtime_init::runtime_init();
    } else {
        wait_forever();
    }
}

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}

pub use asm::nop;

pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        asm::nop();
    }
}

pub fn timer() -> &'static impl interface::time::Timer {
    &TIMER
}
