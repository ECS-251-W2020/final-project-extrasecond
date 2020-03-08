#![feature(format_args_nl)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(exclusive_range_pattern)]

mod arch;
mod bsp;
mod interface;
mod memory;
mod panic;
mod print;
mod runtime_init;

unsafe fn kernel_init() -> ! {
    for i in bsp::device_drivers().iter() {
        if let Err(()) = i.init() {
            panic!("Error loading driver: {}", i.compatible())
        }
    }
    bsp::post_driver_init();

    kernel_main()
}

fn kernel_main() -> ! {
    use interface::console::ConsoleAll;
    use interface::gpio::GPIOAll;

    loop {
        if bsp::console().read_char() == '\n' {
            break;
        }
    }

    println!("[Info] Board name: {}", bsp::board_name());

    println!("[Info] Drivers loaded:");
    for (i, driver) in bsp::device_drivers().iter().enumerate() {
        println!("    {}. {}", i + 1, driver.compatible());
    }

    println!("[Info] {} chars written", bsp::console().chars_written());
    println!("[Info] Echoing input");

    bsp::gpio().setup(0, 1, interface::gpio::Pud::PudOff);
    bsp::gpio().output(0, 1);
    bsp::gpio().input(1);

    bsp::gpio().setup(1, 1, interface::gpio::Pud::PudUp);
    bsp::gpio().setup(2, 1, interface::gpio::Pud::PudDown);

    loop {
        let c = bsp::console().read_char();
        bsp::console().write_char(c);
    }
}
