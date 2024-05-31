use std::{io, time::Duration};

use crate::{pwm_chip::PwmChip, sysfs};

#[derive(Clone)]
pub struct PwmChannel {
    chip: PwmChip,
    channel: u8,
}

impl PwmChannel {
    pub fn new(chip: PwmChip, channel: u8) -> Self {
        let ele = PwmChannel {
            chip,
            channel,
        };
        let _ = ele.export();
        ele
    }

    pub fn set_period(&self, period_ns: Duration) -> Result<(), io::Error> {
        let p = period_ns.as_nanos() as u64;
        sysfs::set_period(self.chip.0, self.channel, p)
    }

    pub fn set_duty_cycle(&self, duty_cycle_ns: Duration) -> Result<(), io::Error> {
        let d = duty_cycle_ns.as_nanos() as u64;
        sysfs::set_duty_cycle(self.chip.0, self.channel, d)
    }

    pub fn set_polarity(&self, polarity: sysfs::Polarity) {
        sysfs::set_polarity(self.chip.0, self.channel, polarity).unwrap();
    }

    fn export(&self) -> Result<(), io::Error> {
        sysfs::export(self.chip.0, self.channel)?;
        Ok(())
    }
    
    fn unexport(&self) -> Result<(), io::Error> {
        sysfs::unexport(self.chip.0, self.channel)?;
        Ok(())
    }

    pub fn enable(&self) -> Result<(), io::Error> {
        sysfs::enable(self.chip.0, self.channel)?;
        Ok(())
    }
    
    pub fn disable(&self) -> Result<(), io::Error> {
        sysfs::disable(self.chip.0, self.channel)?;
        Ok(())
    }

}

// TODO: Implement Drop for PwmChannel(Bug)
impl Drop for PwmChannel{
    fn drop(&mut self) {
        let _ = self.set_duty_cycle(Duration::from_nanos(0));
        let _ = self.set_period(Duration::from_nanos(10000));
        self.set_polarity(sysfs::Polarity::Normal);
        let _ = self.disable();
        let _ = self.unexport();
    }
    
}