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
        console::ConsoleAll,
        gpio::{Dir, GPIOAll, Pud},
        time::Timer,
    };
    use core::time::Duration;

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

    // Wake up core 1.
    if arch::get_core_id() == bsp::CORE_0_ID{
        activate_other_cores();
    }

    bsp::gpio().setup(17, Dir::Output, Pud::PudOff);
    //info!("0x{:08x}", bsp::gpio().input(0));
    bsp::gpio().setup(2, Dir::Input, Pud::PudOff);
    //info!("0x{:08x}", bsp::gpio().input(0));
    let mut i = 0;
    loop {
        if i % 2 == 0 {
            bsp::gpio().output(17, 1);
        } else {
            bsp::gpio().output(17, 0);
        }
        info!("Spinning for 1 second");
  
        arch::timer().spin_for(Duration::from_secs(1));
        i += 1;
    }
}

fn activate_other_cores() {
    use bsp::*;
    SLAVE_CORES_WAKEUP_ADDR.iter().enumerate().for_each(|(i, &addr)| {
        info!("Writing to 0x{:08x} to activate core {}...", addr, i);
        unsafe {
            let dest: *mut u64 = addr as *mut u64;
            *dest = arch::_start as *const () as u64;
            asm!("sev");
        }
    });
}