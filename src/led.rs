use gpio::GpioOut;
use gpio::sysfs::SysFsGpioOutput;

const INCREMENT: u16 = 64;  // 0x40 automatic address increment
const ADDRESS: u16 = 192; // 0xC0 address command
const DSP_PIN: u16 = 128; // 0x80 display control command
const DSP_ON: u16 = 8; // 0x08 display on

// GPIO Pins
const DIO_PIN: u16 = 586;
const CLK_PIN: u16 = 585;
const STB_LEFT: u16 = 581;
const STB_RIGHT: u16 = 582;


struct GpioController {
    dio: SysFsGpioOutput,
    clk: SysFsGpioOutput,
    stb: SysFsGpioOutput,
}

impl GpioController {
    fn new(stb_pin: u16) -> Self {
        let mut dio = SysFsGpioOutput::open(DIO_PIN).unwrap();
        let mut clk = SysFsGpioOutput::open(CLK_PIN).unwrap();
        let mut stb = SysFsGpioOutput::open(stb_pin).unwrap();

        dio.set_low();
        clk.set_high();
        stb.set_high();

        GpioController { dio, clk, stb }
    }

    fn set_stb_low(&mut self) {
        self.stb.set_low();
    }

    fn set_stb_high(&mut self) {
        self.stb.set_high();
    }

    fn write_byte(&mut self, byte_data: u16) {
        for bit in 0..8 {
            self.clk.set_low();
            let bit_value = (byte_data >> bit) & 1;
            self.dio.set_value(bit_value == 1);
            self.clk.set_high();
        }
    }
}



pub(crate) struct LED {
    led_l: GpioController,
    led_r: GpioController,
}

impl LED {
    pub fn new() -> Self {
        Self {
            led_l: GpioController::new(STB_LEFT),
            led_r: GpioController::new(STB_RIGHT),
        }
    }

    // 写左屏幕
    pub fn display_l(&mut self,pos: u16, data: &[u16]) {
        self.led_l.set_stb_high();
        write_data_cmd(&mut self.led_l);
        self.led_l.set_stb_low();
        write(&mut self.led_l, data, pos);
        self.led_l.set_stb_high();
    }

    // 写右屏幕
    pub fn display_r(&mut self,pos: u16, data: &[u16]) {
        self.led_r.set_stb_high();
        write_data_cmd(&mut self.led_r);
        self.led_r.set_stb_low();
        write(&mut self.led_r, data, pos);
        self.led_r.set_stb_high();
    }


}


// 1. setup
fn setup_gpio() {
    // gpio_stb.set_high();
    // TODO 在写入之前再拉高 display_r, l
}

// 2. dsp ctl
fn write_dsp_ctrl(gpio: &mut GpioController, brightness: u16) {
    if !(0..=7).contains(&brightness) {
        panic!("Brightness out of range");
    }
    command(gpio, DSP_PIN | DSP_ON | brightness);
}

// 3. write
fn write(gpio: &mut GpioController, data: &[u16], pos: u16) {

    if !(0..=14).contains(&pos) {
        panic!("Position out of range");
    }
    // write_dsp_ctrl(stb, 5); // 似乎是多出来的
    write_data_cmd(gpio);
    gpio.set_stb_low();
    set_address(gpio, pos);
    for &b in data {
        gpio.write_byte(b);
    }
    gpio.set_stb_high();
}

fn command(gpio: &mut GpioController, cmd: u16) {
    gpio.set_stb_low();
    gpio.write_byte(cmd);
    gpio.set_stb_high();

}

fn write_data_cmd(gpio: &mut GpioController) {
    command(gpio, INCREMENT);
}

fn set_address(gpio: &mut GpioController, addr: u16) {
    gpio.write_byte(ADDRESS| addr);
}


