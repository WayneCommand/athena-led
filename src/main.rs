mod led;

use std::env;
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

    pub fn led_test(&mut self, pos: u16, val: u16) {
        // 小于等于 13，写左边的屏幕，否则写右边的
        if pos <= 13 {
            self.led.display_l(pos, &[val]);
        } else {
            let pos_r = pos - 14u16;
            self.led.display_r(pos_r, &[val]);
        }
    }
}

fn mode(opt: Box<str>) {
    let option = opt.as_ref();
    let mut display = Display::new();

    match option {
        "post" => {
            display.led_post(10 * 1000);

        }
        "clear" => {
            display.clear()
        }
        "libwrt" => {
            display.libwrt();

        }
        _ => {
            display.led_post(10 * 1000);
        }
    }
}

fn testfn(pos: u16, val: u16) {
    let mut display = Display::new();

    display.led_test(pos, val);
}

fn parse_string_to_u16(str: &String) -> u16 {
    let string_ref = str.as_str().parse::<u16>();
    string_ref.unwrap_or_else(|e| 0)
}

fn main() {
    let debug = false;
    // index 1: cmd
    // index 2: cmd arg
    // Collect the command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("args not correct.");
        println!();
        println!("use: athena-led command [options]");
        println!();
        println!("command: -mode, options: post, clear, libwrt.");
        println!();
        println!("command: -test, options: [pos] [val]. e.g: `athena-led -test 0 0xff`");
        println!();
        println!("command: -csv,  options: [file]. e.g: `athena-led -csv ./logo.csv`");
        return;
    };

    let cmd = args.get(1).unwrap().as_str();
    match cmd {
        "-mode" => {
            if args.len() < 3 { println!("wrong mode args.") }
            let mode_opt = String::from(args.get(2).unwrap());

            mode(Box::from(mode_opt.clone()));

            if debug {
                println!("mode = {}",mode_opt.clone());
            }
        }
        "-test" => {
            if args.len() < 4 { println!("wrong mode args.") }
            let opt = parse_string_to_u16(args.get(2).unwrap());
            let val = parse_string_to_u16(args.get(3).unwrap());
            testfn(opt, val);
        }
        _ => {
            println!("not support yet.")
        }
    }

    // Print each argument
    for (index, argument) in args.iter().enumerate() {
        if debug {
            println!("Argument {}: {}", index, argument);
        }
    }

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

