pub fn write_byte(byte: u8) {
    let uart = 0x900_0000 as *mut u32;

    unsafe {
        core::ptr::write_volatile(uart, byte as u32);
    }
}

pub fn write_str(message: &str) {
    for byte in message.bytes() {
        write_byte(byte);
    }
}
