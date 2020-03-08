#![feature(format_args_nl)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(trait_alias)]

mod arch;
mod bsp;
mod interface;
mod memory;
mod panic;
mod print;
mod relocate;
mod runtime_init;

unsafe fn kernel_init() -> ! {
    for i in bsp::device_drivers().iter_mut() {
        if let Err(()) = i.init() {
            panic!("Error loading driver: {}", i.compatible())
        }
    }
    bsp::post_driver_init();
    kernel_main()
}

fn kernel_main() -> ! {
    use interface::console::All;
    use interface::time::Timer;
    use core::time::Duration;

    info!("Booting on: {}", bsp::board_name());

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

    info!("Timer test, spinning for 1 second");
    arch::timer().spin_for(Duration::from_secs(1));

    info!("Echoing input now");
    loop {
        let c = bsp::console().read_char();
        bsp::console().write_char(c);
    }
}
