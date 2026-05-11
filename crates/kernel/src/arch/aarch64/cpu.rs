pub fn wait_for_event() {
    unsafe {
        core::arch::asm!("wfe");
    }
}
