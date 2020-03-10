use crate::memory;
use core::ops::Range;

unsafe fn bss_range() -> Range<*mut usize> {
    extern "C" {
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    Range {
        start: &mut __bss_start,
        end: &mut __bss_end,
    }
}

#[inline(always)]
unsafe fn zero_bss() {
    memory::zero_volatile(bss_range());
}

use crate::arch::{get_core_id, wait_forever};
use crate::bsp::*;

pub unsafe fn runtime_init() -> ! {
    let core_id = get_core_id();
    if CORE_0_ID == core_id {
        zero_bss();

        crate::kernel_init()
    }
    // Should not got here, but just in case.
    wait_forever(core_id)
}
