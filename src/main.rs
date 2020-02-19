#![feature(asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]

#![doc(html_logo_url = "https://git.io/JeGIp")]

mod arch;
mod runtime_init;
mod memory;
mod panic;
mod print;
mod bsp;
mod interface;


/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    println!("Hello from Rust!");
    panic!("Kernel panicked");
}