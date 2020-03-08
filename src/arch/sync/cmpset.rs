global_asm!(include_str!("./cmpset.S"));

use core::ffi::c_void;
extern "C" {
    fn atomic_cmpset(p: *mut c_void, c: i32, t: i32) -> i32;
}

pub fn compare_and_set(lock: *mut bool, new: bool, old: bool) -> bool {
    unsafe { atomic_cmpset(lock as *mut c_void, new as i32, old as i32) != 0 }
}

