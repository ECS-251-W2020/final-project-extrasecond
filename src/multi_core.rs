use crate::arch::{self, init_mmu, sleep, Mutex};
use crate::info;
use core::result::Result;
use core::time::Duration;

static JOB_PTRS: [Mutex<Option<fn()>>; 4] = [
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
];

pub unsafe fn other_cores_main() -> ! {
    let id = arch::get_core_id() as usize;
    init_mmu();
    info!("Core {} init finished.", id);

    loop {
        let mut addr_lock = JOB_PTRS[id].lock();
        if addr_lock.is_none() {
            sleep(Duration::from_millis(100));
        } else {
            let func = addr_lock.unwrap();
            info!("Core {}: Got job, addr: {:x}.", id, func as usize);
            *addr_lock = None;
            drop(addr_lock);
            func();
            info!("Core {}: Jobs done.", id);
        }
    }
}

#[allow(dead_code)]
pub fn submit_job(func: fn(), id: u64) -> Result<(), &'static str> {
    let id = id as usize;
    let mut addr_lock = JOB_PTRS[id].lock();
    if addr_lock.is_some() {
        Err("Had on going job, refuse to override.")
    } else {
        *addr_lock = Some(func);
        Ok(())
    }
}

#[allow(dead_code)]
pub fn submit_job_on_busy(func: fn(), id: u64) {
    let id = id as usize;
    loop {
        let mut addr_lock = JOB_PTRS[id].lock();
        if addr_lock.is_some() {
            sleep(Duration::from_millis(100));
        } else {
            *addr_lock = Some(func);
            break;
        }
    }
}

#[allow(dead_code)]
pub fn submit_job_override(func: fn(), id: u64) -> bool {
    let id = id as usize;
    let mut addr_lock = JOB_PTRS[id].lock();
    let ret = addr_lock.is_some();
    *addr_lock = Some(func);
    ret
}
