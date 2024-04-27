use crate::sysfs;
use crate::pwm_channel::PwmChannel;

#[derive(Clone, Copy)]
pub struct PwmChip(pub u8);

impl PwmChip {
    pub fn new(chip: u8) -> Self {
        PwmChip(chip)
    }

    pub fn get_channel(self, channel: u8) -> PwmChannel {
        PwmChannel::new(self, channel)
    }

    pub fn get_channel_count(self) -> u8 {
        sysfs::get_channel_count(self.0).unwrap()
        
    }
}