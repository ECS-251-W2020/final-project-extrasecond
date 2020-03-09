use super::memory_map;
use crate::memory::*;
use core::ops::RangeInclusive;

pub const NUM_MEM_RANGES: usize = 2;

pub static LAYOUT: KernelVirtualLayout<{ NUM_MEM_RANGES }> = KernelVirtualLayout::new(
    memory_map::mmio::END_INCLUSIVE,
    [
        RangeDescriptor {
            name: "Kernel code and RO data",
            virtual_range: || {
                extern "C" {
                    static __ro_start: usize;
                    static __ro_end: usize;
                }
                unsafe {
                    #[allow(clippy::range_minus_one)]
                    RangeInclusive::new(
                        &__ro_start as *const _ as usize,
                        &__ro_end as *const _ as usize - 1,
                    )
                }
            },
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadOnly,
                execute_never: false,
            },
        },
        RangeDescriptor {
            name: "Device MMIO",
            virtual_range: || {
                RangeInclusive::new(memory_map::mmio::BASE, memory_map::mmio::END_INCLUSIVE)
            },
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            },
        },
    ],
);
