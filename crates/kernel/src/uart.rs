const UART_BASE: usize = 0x900_0000;

const UART_DR: usize = UART_BASE + 0x00;
const UART_FR: usize = UART_BASE + 0x18;

const UART_FR_TXFF: u32 = 1 << 5;

pub fn write_line(message: &str) {
    write_bytes(message.as_bytes());
    write_byte(b'\n');
}

fn write_bytes(message: &[u8]) {
    for byte in message {
        write_byte(*byte);
    }
}

fn write_byte(byte: u8) {
    wait_until_ready_to_send();
    let data_register = UART_DR as *mut u32;

    unsafe {
        core::ptr::write_volatile(data_register, byte as u32);
    }
}

fn wait_until_ready_to_send() {
    while transmit_fifo_is_full() {
        core::hint::spin_loop();
    }
}

fn transmit_fifo_is_full() -> bool {
    let flag_register = UART_FR as *const u32;
    let flags = unsafe { core::ptr::read_volatile(flag_register) };
    flags & UART_FR_TXFF != 0
}
