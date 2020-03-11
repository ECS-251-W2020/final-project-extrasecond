mod memory_map;
mod virt_mem_layout;

use super::driver;
use crate::interface;
use crate::memory::KernelVirtualLayout;
use core::fmt;

pub const CORE_0_ID: u64 = 0;
//pub const CORE_1_ID: u64 = 1;
//pub const CORE_2_ID: u64 = 2;
//pub const CORE_3_ID: u64 = 3;

pub const BOOT_CORE_STACK_START: u64 = 0x80_000;
// pub const CORE_1_STACK_START: u64 = 0x100_000;
/*
/// The address on which the RPi3 firmware loads every binary by default.
pub const BOARD_DEFAULT_LOAD_ADDRESS: usize = 0x80_000;
*/

static mut GPIO: driver::GPIO = unsafe { driver::GPIO::new(memory_map::mmio::GPIO_BASE) };

static mut PL011_UART: driver::PL011Uart =
    unsafe { driver::PL011Uart::new(memory_map::mmio::PL011_UART_BASE) };

static mut PWM: driver::PWM = 
    unsafe { driver::PWM::new(memory_map::mmio::PWM_BASE, memory_map::mmio::CLOCK_BASE) };

pub fn board_name() -> &'static str {
    "Raspberry Pi 3"
}

pub fn console() -> &'static mut impl interface::console::All {
    unsafe { &mut PL011_UART }
}

pub fn gpio() -> &'static mut impl interface::gpio::All {
    unsafe { &mut GPIO }
}

pub fn pwm() -> &'static mut impl interface::pwm::All {
    unsafe { &mut PWM }
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
    unsafe {
        GPIO.map_pl011_uart();
    }
}

/// Return a reference to the virtual memory layout.
pub fn virt_mem_layout() -> &'static KernelVirtualLayout<{ virt_mem_layout::NUM_MEM_RANGES }> {
    &virt_mem_layout::LAYOUT
}

/// Return the address space size in bytes.
pub const fn addr_space_size() -> usize {
    memory_map::mmio::END_INCLUSIVE + 1
}
