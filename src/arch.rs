global_asm!(include_str!("boot.S"));

#[inline(always)]
pub fn wait_forever() -> ! {
    unsafe {
        loop {
            asm!("wfe" :::: "volatile")
        }
    }
}
