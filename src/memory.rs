use core::ops::{Range, RangeInclusive};
use core::fmt;


pub unsafe fn zero_volatile<T>(range: Range<*mut T>)
where
    T: From<u8>,
{
    let mut ptr = range.start;

    while ptr < range.end {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Translation {
    Identity,
    Offset(usize),
}

#[derive(Copy, Clone)]
pub enum MemAttributes {
    CacheableDRAM,
    Device,
}

#[derive(Copy, Clone)]
pub enum AccessPermissions {
    ReadOnly,
    ReadWrite,
}

#[derive(Copy, Clone)]
pub struct AttributeFields {
    pub mem_attributes: MemAttributes,
    pub acc_perms: AccessPermissions,
    pub execute_never: bool,
}

impl Default for AttributeFields {
    fn default() -> Self {
        Self {
            mem_attributes: MemAttributes::CacheableDRAM,
            acc_perms: AccessPermissions::ReadWrite,
            execute_never: true,
        }
    }
}

pub struct RangeDescriptor {
    pub name: &'static str,
    pub virtual_range: fn() -> RangeInclusive<usize>,
    pub translation: Translation,
    pub attribute_fields: AttributeFields,
}

impl fmt::Display for RangeDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Call the function to which self.range points, and dereference the result, which causes
        // Rust to copy the value.
        let start = *(self.virtual_range)().start();
        let end = *(self.virtual_range)().end();
        let size = end - start + 1;

        // log2(1024).
        const KIB_RSHIFT: u32 = 10;

        // log2(1024 * 1024).
        const MIB_RSHIFT: u32 = 20;

        let (size, unit) = if (size >> MIB_RSHIFT) > 0 {
            (size >> MIB_RSHIFT, "MiB")
        } else if (size >> KIB_RSHIFT) > 0 {
            (size >> KIB_RSHIFT, "KiB")
        } else {
            (size, "Byte")
        };

        let attr = match self.attribute_fields.mem_attributes {
            MemAttributes::CacheableDRAM => "C",
            MemAttributes::Device => "Dev",
        };

        let acc_p = match self.attribute_fields.acc_perms {
            AccessPermissions::ReadOnly => "RO",
            AccessPermissions::ReadWrite => "RW",
        };

        let xn = if self.attribute_fields.execute_never {
            "PXN"
        } else {
            "PX"
        };

        write!(
            f,
            "      {:#010x} - {:#010x} | {: >3} {} | {: <3} {} {: <3} | {}",
            start, end, size, unit, attr, acc_p, xn, self.name
        )
    }
}

pub struct KernelVirtualLayout<const NUM_SPECIAL_RANGES: usize>{
    max_virt_addr_inclusive: usize,
    inner: [RangeDescriptor; NUM_SPECIAL_RANGES],
}

impl<const NUM_SPECIAL_RANGES: usize> KernelVirtualLayout<{NUM_SPECIAL_RANGES}> {
    pub const fn new(max: usize, layout: [RangeDescriptor; NUM_SPECIAL_RANGES]) -> Self{
        Self {
            max_virt_addr_inclusive: max,
            inner: layout,
        }
    }
    pub fn get_virtual_addr_properties(&self, virt_addr: usize) -> Result<(usize, AttributeFields), &'static str>{
        if virt_addr > self.max_virt_addr_inclusive {
            return Err("Address out of bound");
        }
        for i in self.inner.iter() {
            if (i.virtual_range)().contains(&virt_addr){
                let output_addr = match i.translation {
                    Translation::Identity => virt_addr,
                    Translation::Offset(a) => a + (virt_addr - (i.virtual_range)().start()),
                };
                return Ok((output_addr, i.attribute_fields));
            }
        }
        Ok((virt_addr, AttributeFields::default()))
    }
    pub fn print_layout(&self) {
        use crate::info;

        for i in self.inner.iter() {
            info!("{}", i);
        }
    }
}

impl<const NUM_SPECIAL_RANGES: usize> fmt::Display for KernelVirtualLayout<{NUM_SPECIAL_RANGES}>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Max Virtual Address: {}", self.max_virt_addr_inclusive)?;
        for i in self.inner.iter(){
            writeln!(f, "{}", i)?; 
        }
        Ok(())
    }
}