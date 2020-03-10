#![feature(format_args_nl)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(exclusive_range_pattern)]
#![allow(incomplete_features)]
#![feature(const_generics)]

mod arch;
mod bsp;
mod interface;
mod memory;
mod panic;
mod print;
mod runtime_init;

unsafe fn kernel_init() -> ! {
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
    kernel_main()
}

fn kernel_main() -> ! {
    use crate::interface::console::ConsoleAll;
    use core::time::Duration;
    use interface::gpio::GPIOAll;
    use interface::time::Timer;

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

    bsp::gpio().setup(0, 1, interface::gpio::Pud::PudOff);
    info!(bsp::gpio().output(0, 1));
    info!(bsp::gpio().input(1));

    bsp::gpio().setup(1, 1, interface::gpio::Pud::PudUp);
    bsp::gpio().setup(2, 1, interface::gpio::Pud::PudDown);

    loop {
        info!("Spinning for 1 second");
        arch::timer().spin_for(Duration::from_secs(1));
    }

    /*    info!("Echoing input now");

    loop {
        let c = bsp::console().read_char();
        bsp::console().write_char(c);
    }*/
}
