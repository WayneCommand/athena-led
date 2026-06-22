use gpio::GpioOut;
use gpio::sysfs::SysFsGpioOutput;

const CMD_AUTO_INCR: u8 = 0x40;
const CMD_ADDRESS: u8 = 0xC0;

const PIN_DIO: u16 = 586;
const PIN_CLK: u16 = 585;
const PIN_STB_LEFT: u16 = 581;
const PIN_STB_RIGHT: u16 = 582;

struct GpioPins {
    dio: SysFsGpioOutput,
    clk: SysFsGpioOutput,
    stb: SysFsGpioOutput,
}

impl GpioPins {
    fn new(stb_pin: u16) -> Self {
        let mut dio = SysFsGpioOutput::open(PIN_DIO).unwrap();
        let mut clk = SysFsGpioOutput::open(PIN_CLK).unwrap();
        let mut stb = SysFsGpioOutput::open(stb_pin).unwrap();
        dio.set_low();
        clk.set_high();
        stb.set_high();
        GpioPins { dio, clk, stb }
    }

    fn stb_low(&mut self) {
        self.stb.set_low();
    }

    fn stb_high(&mut self) {
        self.stb.set_high();
    }

    fn write_byte(&mut self, data: u8) {
        for bit in 0..8 {
            self.clk.set_low();
            self.dio.set_value((data >> bit) & 1 == 1);
            self.clk.set_high();
        }
    }
}

pub(crate) struct LED {
    left: GpioPins,
    right: GpioPins,
}

impl LED {
    pub fn new() -> Self {
        LED {
            left: GpioPins::new(PIN_STB_LEFT),
            right: GpioPins::new(PIN_STB_RIGHT),
        }
    }

    pub fn display_left(&mut self, pos: u8, data: &[u8]) {
        send_command(&mut self.left, CMD_AUTO_INCR);
        write_data(&mut self.left, data, pos);
    }

    pub fn display_right(&mut self, pos: u8, data: &[u8]) {
        send_command(&mut self.right, CMD_AUTO_INCR);
        write_data(&mut self.right, data, pos);
    }
}

fn send_command(pins: &mut GpioPins, cmd: u8) {
    pins.stb_low();
    pins.write_byte(cmd);
    pins.stb_high();
}

fn write_data(pins: &mut GpioPins, data: &[u8], start_addr: u8) {
    pins.stb_low();
    pins.write_byte(CMD_ADDRESS | start_addr);
    for &byte in data {
        pins.write_byte(byte);
    }
    pins.stb_high();
}
