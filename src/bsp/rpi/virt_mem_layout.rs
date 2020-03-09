use super::memory_map;
use crate::memory::*;
use core::ops::RangeInclusive;

pub const NUM_MEM_RANGES: usize = 3;

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
            }
        },
        RangeDescriptor {
            name: "Remapped Device MMIO",
            virtual_range: || {
                // The last 64 KiB slot in the first 512 MiB
                RangeInclusive::new(0x1FFF_0000, 0x1FFF_FFFF)
            },
            translation: Translation::Offset(memory_map::mmio::BASE + 0x20_0000),
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
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
    ]
);