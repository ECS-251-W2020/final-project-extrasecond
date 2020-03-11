mod memory_map;
mod virt_mem_layout;

use super::driver;
use crate::interface;
use crate::memory::KernelVirtualLayout;
use core::fmt;

#[allow(dead_code)]
pub const CORE_0_ID: u64 = 0;
#[allow(dead_code)]
pub const CORE_1_ID: u64 = 1;
#[allow(dead_code)]
pub const CORE_2_ID: u64 = 2;
#[allow(dead_code)]
pub const CORE_3_ID: u64 = 3;

// When these addresses are non-zero, corresponding
// core will use the non-zero value as an address and
// jump to there, otherwise it is wfe. This is how we
// wake up cores by giving it an function entry.
#[allow(dead_code)]
pub const MASTER_CORE_WAKEUP_ADDR: u64 = 0xd8;
pub const SLAVE_CORES_WAKEUP_ADDR: [u64; 3] = [0xe0, 0xe8, 0xf0];

pub const BOOT_CORE_STACK_START: u64 = 0x80_000;

// 16k stack for slave cores
pub const SLAVE_STACK_SHIFT: u64 = 14;
pub const SLAVE_STACK_PREAMBLE: u64 = 0b1101_00;
#[allow(dead_code)]
pub const CORE_1_STACK_START: u64 = 0x0D4_000; // 0b1010_01_00_0000_...
#[allow(dead_code)]
pub const CORE_2_STACK_START: u64 = 0x0D8_000; // 0b1010_10_00_0000_...
#[allow(dead_code)]
pub const CORE_3_STACK_START: u64 = 0x0DB_000; // 0b1010_11_00_0000_...

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

pub fn console() -> &'static mut impl interface::console::ConsoleAll {
    unsafe { &mut PL011_UART }
}

pub fn gpio() -> &'static mut impl interface::gpio::GPIOAll {
    unsafe { &mut GPIO }
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
