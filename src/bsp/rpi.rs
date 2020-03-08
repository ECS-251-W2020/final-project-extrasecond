mod memory_map;

use super::driver;
use crate::interface;
use core::fmt;

pub const BOOT_CORE_ID: u64 = 0;
pub const BOOT_CORE_STACK_START: u64 = 0x80_000;

/*
/// The address on which the RPi3 firmware loads every binary by default.
pub const BOARD_DEFAULT_LOAD_ADDRESS: usize = 0x80_000;
*/

static mut GPIO: driver::GPIO = unsafe { driver::GPIO::new(memory_map::mmio::GPIO_BASE) };

static mut PL011_UART: driver::PL011Uart =
    unsafe { driver::PL011Uart::new(memory_map::mmio::PL011_UART_BASE) };

pub fn board_name() -> &'static str {
    "Raspberry Pi 3"
}

pub fn console() -> &'static mut impl interface::console::All {
    unsafe { &mut PL011_UART }
}

pub unsafe fn panic_console_out() -> impl fmt::Write {
    let uart = driver::PanicUart::new(memory_map::mmio::PL011_UART_BASE);
    uart.init();
    uart
}

pub fn device_drivers() -> [&'static mut dyn interface::driver::DeviceDriver; 2] {
    unsafe { [&mut GPIO, &mut PL011_UART] } 
}

pub fn post_driver_init() {
    unsafe { GPIO.map_pl011_uart(); }
}
