mod exception;
mod mmu;
pub mod sync;
mod time;

use crate::{bsp, interface};
use cortex_a::{asm, regs::*};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    use crate::runtime_init::{master_core_init, other_cores_init};
    match get_core_id() {
        // Core 0: Master core
        0b00 => el2_to_el1_transition(master_core_init as *const () as u64, bsp::BOOT_CORE_STACK_START),

        // Core 1-3: Slave core
        0b01 | 0b10 | 0b11 => el2_to_el1_transition(other_cores_init as *const () as u64, (0b1011_00 | get_core_id()) << bsp::SLAVE_CORE_STACK_SHIFT),

        // Should not happen
        _ => wait_forever(get_core_id())
    }
    
}

#[inline(always)]
unsafe fn el2_to_el1_transition(next_func_addr: u64, stack_start_addr: u64) -> ! {
    // Enable timer counter registers for EL1.
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading the counters.
    CNTVOFF_EL2.set(0);

    // Set EL1 execution state to AArch64.
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // Set up a simulated exception return.
    //
    // First, fake a saved program status, where all interrupts were masked and SP_EL1 was used as a
    // stack pointer.
    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    // Second, let the link register point to init().
    ELR_EL2.set(next_func_addr);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it.
    SP_EL1.set(stack_start_addr);

    // Use `eret` to "return" to EL1. This will result in execution of `reset()` in EL1.
    asm::eret()
}

#[inline(always)]
pub fn wait_forever(core_id: u64) -> ! {
    loop {
        asm::wfe();
    }
}

pub use asm::nop;

pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        asm::nop();
    }
}

static TIMER: time::Timer = time::Timer;
pub fn timer() -> &'static impl interface::time::Timer {
    &TIMER
}

#[inline(always)]
pub fn get_core_id() -> u64 {
    const CORE_MASK: u64 = 0x3; // The last two bits for 4 cores
    MPIDR_EL1.get() & CORE_MASK
}
/// Information about the HW state.
pub mod state {
    use crate::arch::PrivilegeLevel;
    use cortex_a::regs::*;

    /// The processing element's current privilege level.
    pub fn current_privilege_level() -> (PrivilegeLevel, &'static str) {
        let el = CurrentEL.read_as_enum(CurrentEL::EL);
        match el {
            Some(CurrentEL::EL::Value::EL2) => (PrivilegeLevel::Hypervisor, "EL2"),
            Some(CurrentEL::EL::Value::EL1) => (PrivilegeLevel::Kernel, "EL1"),
            Some(CurrentEL::EL::Value::EL0) => (PrivilegeLevel::User, "EL0"),
            _ => (PrivilegeLevel::Unknown, "Unknown"),
        }
    }

    /// Print the AArch64 exceptions status.
    #[rustfmt::skip]
    pub fn print_exception_state() {
        use super::{
            exception,
            exception::{Debug, SError, FIQ, IRQ},
        };
        use crate::info;

        let to_mask_str = |x| -> _ {
            if x { "Masked" } else { "Unmasked" }
        };

        info!("      Debug:  {}", to_mask_str(exception::is_masked::<Debug>()));
        info!("      SError: {}", to_mask_str(exception::is_masked::<SError>()));
        info!("      IRQ:    {}", to_mask_str(exception::is_masked::<IRQ>()));
        info!("      FIQ:    {}", to_mask_str(exception::is_masked::<FIQ>()));
    }
}

static MMU: mmu::MMU = mmu::MMU;

pub fn mmu() -> &'static impl interface::mm::MMU {
    &MMU
}
