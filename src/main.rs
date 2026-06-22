mod led;
mod display;

use std::env;
use display::Display;

fn print_usage() -> ! {
    println!("Usage: athena-led <command> [options]");
    println!();
    println!("Commands:");
    println!("  -mode post      Power-on self-test (10s)");
    println!("  -mode clear     Clear all LEDs");
    println!("  -mode libwrt    Display \"LibW\" logo");
    println!("  -test <pos> <val>  Set pixel at position 0-27 to value (hex ok)");
    println!("  -csv  <file>    Load pattern from CSV (not yet implemented)");
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
    }

    match args[1].as_str() {
        "-mode" => {
            if args.len() < 3 {
                println!("Error: missing mode argument");
                print_usage();
            }
            let mut display = Display::new();
            match args[2].as_str() {
                "post" => display.self_test(10_000),
                "clear" => display.clear(),
                "libwrt" => display.show_logo(),
                other => {
                    println!("Error: unknown mode '{}'", other);
                    print_usage();
                }
            }
        }
        "-test" => {
            if args.len() < 4 {
                println!("Error: missing position or value");
                print_usage();
            }
            let pos: u8 = args[2].parse().unwrap_or(0);
            let val: u8 = args[3].parse().unwrap_or(0);
            let mut display = Display::new();
            display.set_pixel(pos, val);
        }
        _ => {
            println!("Error: unknown command '{}'", args[1]);
            print_usage();
        }
    }
}
