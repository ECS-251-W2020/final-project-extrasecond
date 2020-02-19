#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

#![doc(html_logo_url = "https://git.io/JeGIp")]

mod arch;
mod runtime_init;
mod memory;
mod panic;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    panic!();
}