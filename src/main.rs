use gpio::{sysfs::SysFsGpioInput, GpioIn, GpioValue};
use std::{env, io, process::Command, thread::sleep, time::Duration};

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

    /*
    This works around a race condition - we have to wait for permissions to be set correctly by the OS.
    I suspect this hasn't been caught because people usually just use 'sudo'.
    We could submit a PR to 'gpio' but the bigger issue is that the whole interface is deprecated.
        (https://github.com/mbr/gpio-rs/issues/2)
    The 'gpio-cdev' crate uses the modern interface, but is a bit low-level. I've already burnt one pin playing with it.
    */
    let mut gpio = loop {
        match SysFsGpioInput::open(port) {
            Ok(x) => {
                println!("Successfully opened GPIO {}", port);
                break x;
            }
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    println!("Permission error on GPIO {}, trying again", port);
                    sleep(Duration::from_millis(100));
                }
                _ => panic!("Failed to open GPIO: {}", e),
            },
        }
    };

    loop {
        let button_val = gpio.read_value().expect("Failed to read from button");
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
        //TODO with the cdev interface we should just be able to block
        sleep(Duration::from_millis(100));
    }
}
