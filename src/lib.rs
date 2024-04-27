mod sysfs;
pub mod pwm_channel;
pub mod pwm_chip;
pub mod service;

use std::{thread::sleep, time::Duration};

pub use sysfs::reset_all;

use crate::pwm_chip::PwmChip;

// test
pub fn demo() {
    println!("Hello from pwm-rs!");
    let chip = PwmChip::new(0);
    let channel = chip.get_channel(0);
    channel.set_period(Duration::from_micros(20000));
    channel.set_duty_cycle(Duration::from_micros(1000));
    sleep(Duration::from_secs(3))
}