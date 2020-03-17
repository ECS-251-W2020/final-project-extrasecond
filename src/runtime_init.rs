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

pub unsafe fn master_core_init() -> ! {
    zero_bss();

    crate::kernel_main()
}

pub unsafe fn other_cores_init() -> ! {
    crate::other_cores_main()
}
