global_asm!(include_str!("aarch64/start.S"));

#[inline(always)]
pub fn wait_forever() -> ! {
    unsafe {
        loop {
            asm!("wfe" :::: "volatile")
        }
    }
}
