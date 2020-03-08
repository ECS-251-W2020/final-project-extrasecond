#![feature(format_args_nl)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(asm)]
#![feature(global_asm)]

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

    println!(" __  __ _      _ _                 _ ");
    println!("|  \\/  (_)_ _ (_) |   ___  __ _ __| |");
    println!("| |\\/| | | ' \\| | |__/ _ \\/ _` / _` |");
    println!("|_|  |_|_|_||_|_|____\\___/\\__,_\\__,_|");
    println!();
    println!("{:^37}", bsp::board_name());
    println!();
    println!("[ML] Requesting binary");
    bsp::console().flush();

    // Clear the RX FIFOs, if any, of spurious received characters before starting with the loader
    // protocol.
    bsp::console().clear();

    // Notify `Minipush` to send the binary.
    for _ in 0..3 {
        bsp::console().write_char(3 as char);
    }

    // Read the binary's size.
    let mut size: u32 = u32::from(bsp::console().read_char() as u8);
    size |= u32::from(bsp::console().read_char() as u8) << 8;
    size |= u32::from(bsp::console().read_char() as u8) << 16;
    size |= u32::from(bsp::console().read_char() as u8) << 24;

    // Trust it's not too big.
    bsp::console().write_char('O');
    bsp::console().write_char('K');

    let kernel_addr: *mut u8 = bsp::BOARD_DEFAULT_LOAD_ADDRESS as *mut u8;
    unsafe {
        // Read the kernel byte by byte.
        for i in 0..size {
            *kernel_addr.offset(i as isize) = bsp::console().read_char() as u8;
        }
    }

    println!("[ML] Loaded! Executing the payload now\n");
    bsp::console().flush();

    // Use black magic to get a function pointer.
    let kernel: extern "C" fn() -> ! = unsafe { core::mem::transmute(kernel_addr as *const ()) };

    // Jump to loaded kernel!
    kernel()
}
