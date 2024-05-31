use std::fs::File;
use std::io::{Read, Write};
use std::fmt;
use std::{io, path::Path};
use core::fmt::Display;

pub enum Polarity {
    Normal,
    Inverted,
}

impl Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Polarity::Normal => write!(f, "normal"),
            Polarity::Inverted => write!(f, "inverted"),
        }
    }
    
}

pub fn reset_all() {
    // unexport all pwm channels
    let chip_count = get_chip_count().unwrap();
    for chip in 0..chip_count {
        let channel_count = get_channel_count(chip).unwrap();
        for channel in 0..channel_count {
            let _ = export(chip, channel);
            let _ = enable(chip, channel);
            let _ = set_period(chip, channel, 10000);
            let _ = set_duty_cycle(chip, channel, 0);
            let _ = set_polarity(chip, channel, Polarity::Normal);
            let _ = disable(chip, channel);
            let _ = unexport(chip, channel);
        }
    }
}

pub fn export(pwm_chip: u8, pwm_channel: u8) -> Result<(), io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}", pwm_chip);
    let path = Path::new(&path);
    if is_exported(pwm_chip, pwm_channel)? {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "PWM channel is already exported"))
    }
    if path.exists() {
        File::create(path.join("export"))?.write_fmt(format_args!("{}", pwm_channel))?;
        return Ok(());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM chip not found"))
}

pub fn unexport(pwm_chip: u8, pwm_channel: u8) -> Result<(), io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}", pwm_chip);
    let path = Path::new(&path);
    if !is_exported(pwm_chip, pwm_channel)? {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "PWM channel is not exported yet"))
    }
    if path.exists() {
        File::create(path.join("unexport"))?.write_fmt(format_args!("{}", pwm_channel))?;
        return Ok(());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM chip not found"))
}

pub fn is_exported(pwm_chip: u8, pwm_channel: u8) -> Result<bool, io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    Ok(path.exists())
}

pub fn enable(pwm_chip: u8, pwm_channel: u8) -> Result<(), io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    if is_enabled(pwm_chip, pwm_channel)? {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("PWM channel {} on chip {} is already enabled", pwm_channel, pwm_chip)))
    }
    if path.exists() {
        File::create(path.join("enable"))?.write_fmt(format_args!("{}", 1))?;
        return Ok(());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM channel not found"))
}

pub fn disable(pwm_chip: u8, pwm_channel: u8) -> Result<(), io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    if !is_enabled(pwm_chip, pwm_channel)? {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "PWM channel is not enabled yet"))
    }
    if path.exists() {
        File::create(path.join("enable"))?.write_fmt(format_args!("{}", 0))?;
        return Ok(());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM channel not found"))
}

pub fn is_enabled(pwm_chip: u8, pwm_channel: u8) -> Result<bool, io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}/enable", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return Ok(contents.trim() == "1");
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM channel not found"))
}

pub fn set_duty_cycle(pwm_chip: u8, pwm_channel: u8, duty_cycle: u64) -> Result<(), io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    if path.exists() {
        File::create(path.join("duty_cycle"))?.write_fmt(format_args!("{}", duty_cycle))?;
        return Ok(());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM channel not found"))
}

pub fn get_duty_cycle(pwm_chip: u8, pwm_channel: u8) -> Result<u64, io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}/duty_cycle", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return Ok(contents.trim().parse().unwrap());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM channel not found"))
}

pub fn set_period(pwm_chip: u8, pwm_channel: u8, period: u64) -> Result<(), io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    if path.exists() {
        File::create(path.join("period"))?.write_fmt(format_args!("{}", period))?;
        return Ok(());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM channel not found"))
}

pub fn get_period(pwm_chip: u8, pwm_channel: u8) -> Result<u64, io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}/period", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return Ok(contents.trim().parse().unwrap());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM channel not found"))
}

pub fn set_polarity(pwm_chip: u8, pwm_channel: u8, polarity: Polarity) -> Result<(), io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    if path.exists() {
        File::create(path.join("polarity"))?.write_fmt(format_args!("{}", polarity))?;
        return Ok(());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM channel not found"))
}

pub fn get_polarity(pwm_chip: u8, pwm_channel: u8) -> Result<Polarity, io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}/pwm{}/polarity", pwm_chip, pwm_channel);
    let path = Path::new(&path);
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return match contents.trim() {
            "normal" => Ok(Polarity::Normal),
            "inverted" => Ok(Polarity::Inverted),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid polarity value")),
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM channel not found"))
}

pub fn get_chip_count() -> Result<u8, io::Error> {
    let path = Path::new("/sys/class/pwm");
    if path.exists() {
        let mut count = 0;
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                count += 1;
            }
        }
        return Ok(count);
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM chip not found"))
}

pub fn get_channel_count(pwm_chip: u8) -> Result<u8, io::Error> {
    let path = format!("/sys/class/pwm/pwmchip{}", pwm_chip);
    let path = Path::new(&path);
    if path.exists() {
        let buffer = &mut String::new();
        File::open(path.join("npwm"))?.read_to_string(buffer)?;
        let count = buffer.trim().parse().unwrap();
        return Ok(count);
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "PWM chip not found"))
}