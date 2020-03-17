#![feature(format_args_nl)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(exclusive_range_pattern)]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(asm)]
#![feature(global_asm)]

mod arch;
mod bsp;
mod interface;
mod memory;
mod panic;
mod print;
mod runtime_init;

use interface::{
    console::All as ConsoleAll,
    gpio::All as GPIOAll,
    gpio::{Dir, Pud},
    pwm::All as PWMAll,
    time::Timer,
};
use core::time::Duration;

unsafe fn kernel_init() {
    use crate::interface::mm::MMU;

    if let Err(err_msg) = arch::mmu().init() {
        panic!("MMU: {}", err_msg);
    }
    for i in bsp::device_drivers().iter_mut() {
        if let Err(()) = i.init() {
            panic!("Error loading driver: {}", i.compatible())
        }
    }
    bsp::post_driver_init();
}

unsafe fn other_cores_main() -> ! {
    let id = crate::arch::get_core_id();
    info!("Init slave core {}...", id);
    kernel_init();

    info!("Core {} init finished, starting to receive jobs...", id);
    loop {
        info!("Jobs done. sleep for 500 mill seconds...");
        arch::timer().spin_for(Duration::from_millis(500));
    }
}

fn kernel_main() -> ! {
    unsafe {
        kernel_init();
    }

    info!("Hit ENTER to continue...");
    loop {
        if bsp::console().read_char() == '\n' {
            break;
        }
    }

    info!("Booting on: {}", bsp::board_name());

    info!("{}", bsp::virt_mem_layout());

    let (_, privilege_level) = arch::state::current_privilege_level();
    info!("Current privilege level: {}", privilege_level);

    info!("Exception handling state:");
    arch::state::print_exception_state();

    info!(
        "Architectural timer resolution: {} ns",
        arch::timer().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    for (i, driver) in bsp::device_drivers().iter().enumerate() {
        info!("      {}. {}", i + 1, driver.compatible());
    }

    // Wake up slave cores.
    activate_other_cores();

    bsp::gpio().setup(17, Dir::Output, Pud::PudOff);
    bsp::gpio().setup(2, Dir::Input, Pud::PudOff);
    info!("{:032b}", bsp::gpio().input(0));

    bsp::gpio().setup_pwm(12);
    bsp::pwm().write(12, 100);

    let mut i = 0;
    loop {
        if i % 2 == 0 {
            bsp::gpio().output(17, 1);
        } else {
            bsp::gpio().output(17, 0);
        }
        info!("Spinning for 1 second");
        unsafe {
            asm!("sev");
        }
        arch::timer().spin_for(Duration::from_secs(1));
        i += 1;
    }
}

/// Nice and nite activation thanks to rust's zero-abstraction.
fn activate_other_cores() {
    bsp::SLAVE_CORES_WAKEUP_ADDR
        .iter()
        .enumerate()
        .for_each(|(i, &addr)| {
            info!("Writing to 0x{:08x} to activate core {}...", addr, i);
            unsafe {
                // Get the address to activate that core.
                let dest: *mut u64 = addr as *mut u64;
                // Store _start function address as slave core entry point.
                *dest = arch::_start as *const () as u64;
                // Activate!
                asm!("sev");
            }
        });
}
