mod memory_map;

use crate::interface;
use core::fmt;
use super::driver;

pub const BOOT_CORE_ID: u64 = 0;
pub const BOOT_CORE_STACK_START: u64 = 0x80_000;

static GPIO: driver::GPIO = unsafe {
    driver::GPIO::new(memory_map::mmio::GPIO_BASE)
};

static PL011_UART: driver::PL011Uart = unsafe {
    driver::PL011Uart::new(memory_map::mmio::PL011_UART_BASE)
};

pub fn board_name() -> &'static str {
    "Raspberry Pi 3"
}

pub fn console() -> &'static impl interface::console::All {
    &PL011_UART
}

pub unsafe fn panic_console_out() -> impl fmt::Write {
    let uart = driver::PanicUart::new(memory_map::mmio::PL011_UART_BASE);
    uart.init();
    uart
}

pub fn device_drivers() -> [&'static dyn interface::driver::DeviceDriver; 2] {
    [&GPIO, &PL011_UART]
}

pub fn post_driver_init() {
    GPIO.map_pl011_uart();
}
