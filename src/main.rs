#![feature(format_args_nl)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![doc(html_logo_url = "https://git.io/JeGIp")]

mod arch;
mod bsp;
mod interface;
mod memory;
mod panic;
mod print;
mod runtime_init;

unsafe fn kernel_init() -> ! {
    println!("Hello from Rust!");
    panic!("Kernel panicked");
}
