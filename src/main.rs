#![feature(format_args_nl)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(exclusive_range_pattern)]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(asm)]

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
    use crate::interface::{
        gpio::{Dir, Pud},
        time::Timer,
    };
    use core::time::Duration;
    use interface::console::All as ConsoleAll;
    use interface::gpio::All as GPIOAll;
    use interface::pwm::All as PWMAll;

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

    info!("Hit ENTER to continue...");
    loop {
        if bsp::console().read_char() == '\n' {
            break;
        }
    }

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
        info!("Spinning for 1 second, sending an event");
        unsafe {
            asm!("sev");
        }
        arch::timer().spin_for(Duration::from_secs(1));
        i += 1;
    }

    /*    info!("Echoing input now");

    loop {
        let c = bsp::console().read_char();
        bsp::console().write_char(c);
    }*/
}
