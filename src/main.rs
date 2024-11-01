mod led;

use std::thread::sleep;
use std::time::Duration;
use led::LED;

struct Display {
    led: LED
}

impl Display {
    pub fn new() -> Self{
        Self{
            led: LED::new()
        }
    }

    pub fn clear(&mut self) {
        let data = [0x00; 14];
        self.led.display_l(0, &data);
        self.led.display_r(0, &data);
    }

    pub fn fill_all(&mut self) {
        let data = [0xff; 14];
        self.led.display_l(0, &data);
        self.led.display_r(0, &data);
    }

    // 自检
    // timeout: 10 * 1000 = 10s
    pub fn led_post(&mut self, timeout: u64) {
        self.clear();
        self.fill_all();
        sleep(Duration::from_millis(timeout));
    }

    pub fn libwrt(&mut self) {
        self.clear();
        // L
        self.led.display_l(0, &[0x1f, 0x02, 0x10, 0x02]);
        // i
        self.led.display_l(4, &[0xa0, 0x03]);
        // b
        self.led.display_l(6, &[0xe0, 0x03, 0x94, 0x02, 0x08, 0x00]);
        // W - left
        self.led.display_l(12, &[0x1f, 0x02]);
        // W - right
        self.led.display_r(0, &[0x4c, 0x02, 0xC8, 0x00]);
        // r
        self.led.display_r(4, &[0x81, 0x03, 0x88, 0x00]);
        // t
        self.led.display_r(8, &[0x80, 0x00, 0x9e, 0x02]);
    }


}


fn main() {
    let mut display = Display::new();
    display.led_post(10 * 1000);
    display.libwrt();

    // 4
    // 显示是以列顺序，两个 bytes 连在一起 16 个 bit，用于表示两列 10 个 LED，然后忽略最后 6个 0
    // let byte_data = [0x80, 0x00, 0x9e, 0x02];
    // for &byte in &byte_data {
    //     let display = (0..8)
    //         .map(|bit| ((byte >> bit) & 1) as u8)
    //         .collect::<Vec<_>>();
    //     println!("{:?}", display);
    // }
}
