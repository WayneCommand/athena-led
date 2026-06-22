use std::thread::sleep;
use std::time::Duration;
use crate::led::LED;

const LEFT_SIZE: u8 = 14;
const TOTAL_SIZE: u8 = 28;

pub(crate) struct Display {
    led: LED,
}

impl Display {
    pub fn new() -> Self {
        Display { led: LED::new() }
    }

    pub fn clear(&mut self) {
        let blank = [0u8; LEFT_SIZE as usize];
        self.led.display_left(0, &blank);
        self.led.display_right(0, &blank);
    }

    pub fn fill_all(&mut self) {
        let filled = [0xffu8; LEFT_SIZE as usize];
        self.led.display_left(0, &filled);
        self.led.display_right(0, &filled);
    }

    /// Write data at a unified position (0-27), auto-splitting across panels.
    pub fn write(&mut self, pos: u8, data: &[u8]) {
        assert!(pos < TOTAL_SIZE, "position out of range");
        assert!(
            pos + data.len() as u8 <= TOTAL_SIZE,
            "data exceeds screen boundary"
        );

        if pos + data.len() as u8 <= LEFT_SIZE {
            self.led.display_left(pos, data);
        } else if pos >= LEFT_SIZE {
            self.led.display_right(pos - LEFT_SIZE, data);
        } else {
            let left_len = (LEFT_SIZE - pos) as usize;
            self.led.display_left(pos, &data[..left_len]);
            self.led.display_right(0, &data[left_len..]);
        }
    }

    pub fn set_pixel(&mut self, pos: u8, val: u8) {
        self.write(pos, &[val]);
    }

    pub fn self_test(&mut self, duration_ms: u64) {
        self.clear();
        self.fill_all();
        sleep(Duration::from_millis(duration_ms));
    }

    pub fn show_logo(&mut self) {
        self.clear();
        // L i b
        self.write(0, &[0x1f, 0x02, 0x10, 0x02]);
        self.write(4, &[0xa0, 0x03]);
        self.write(6, &[0xe0, 0x03, 0x94, 0x02, 0x08, 0x00]);
        // W (split across left and right panels)
        self.write(12, &[0x1f, 0x02, 0x4c, 0x02, 0xc8, 0x00]);
        // r t
        self.write(18, &[0x81, 0x03, 0x88, 0x00]);
        self.write(22, &[0x80, 0x00, 0x9e, 0x02]);
    }
}
