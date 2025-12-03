//! UART 16550 compatible MMIO driver for QEMU RISC-V.

use spin::Mutex;

const UART_BASE: usize = 0x1000_0000;

bitflags::bitflags! {
    /// Line status flags
    struct LineStsFlags: u8 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}

struct Uart16550 {
    base: usize,
}

impl Uart16550 {
    const fn new(base: usize) -> Self {
        Self { base }
    }

    fn read_register(&self, offset: usize) -> u8 {
        unsafe { core::ptr::read_volatile((self.base + offset) as *const u8) }
    }

    fn write_register(&mut self, offset: usize, value: u8) {
        unsafe { core::ptr::write_volatile((self.base + offset) as *mut u8, value) }
    }

    fn init(&mut self, baud_rate: usize) {
        let clock_freq = 22_729_200usize;
        let divisor = clock_freq / (16 * baud_rate);

        // Disable interrupts
        self.write_register(1, 0x00);

        // Enable DLAB
        self.write_register(3, 0x80);

        // Set baud rate divisor
        self.write_register(0, (divisor & 0xff) as u8);
        self.write_register(1, ((divisor >> 8) & 0xff) as u8);

        // Disable DLAB and set data word length to 8 bits
        self.write_register(3, 0x03);

        // Enable FIFO, clear TX/RX queues and set interrupt watermark at 14 bytes
        self.write_register(2, 0xC7);

        // Mark data terminal ready, signal request to send
        self.write_register(4, 0x0B);
    }

    fn line_sts(&self) -> LineStsFlags {
        LineStsFlags::from_bits_truncate(self.read_register(5))
    }

    fn putchar(&mut self, c: u8) {
        while !self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {
            core::hint::spin_loop();
        }
        self.write_register(0, c);
    }

    fn getchar(&self) -> Option<u8> {
        if self.line_sts().contains(LineStsFlags::INPUT_FULL) {
            Some(self.read_register(0))
        } else {
            None
        }
    }
}

static UART: Mutex<Uart16550> = Mutex::new(Uart16550::new(UART_BASE));

pub fn console_putchar(c: u8) {
    UART.lock().putchar(c);
}

#[allow(dead_code)]
pub fn console_getchar() -> Option<u8> {
    UART.lock().getchar()
}

pub fn init() {
    UART.lock().init(115200);
}
