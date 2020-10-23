use gpio::{sysfs::SysFsGpioInput, GpioIn, GpioValue};
use std::{env, process::Command, thread::sleep, time::Duration};

fn main() {
    let mut args = env::args();
    let _exe_name = args.next();
    let port = args
        .next()
        .expect("Not enough args")
        .parse()
        .expect("Couldn't parse port as int");
    let cmd_name = args.next().expect("Not enough args");
    let cmd_args: Vec<String> = args.collect();

    let mut prev_button_val = GpioValue::High; // assume its off to begin with
    let mut button = SysFsGpioInput::open(port).expect("Failed to open GPIO port");
    loop {
        let button_val = button.read_value().expect("Failed to read from button");
        match button_val {
            GpioValue::High => {}
            GpioValue::Low => {
                if prev_button_val == GpioValue::High {
                    println!("Button pressed - executing command");
                    Command::new(&cmd_name)
                        .args(&cmd_args)
                        .spawn()
                        .expect("Failed to call command");
                }
            }
        }
        prev_button_val = button_val;
        //TODO presumably there's some way to just block?
        sleep(Duration::from_millis(100));
    }
}
