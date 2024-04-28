# Oxy-linux-plus
This is a Rust PWM crate for Linux that automatically selects software or hardware implementation. Hardware PWM will be implemented through the Linux sysfs ABI. Please check if your PWM chip is under /sys/class/pwm directory.
(The software pwm engine has not been done)

``` rust
// the channel will be reset after channel dropped
fn demo() {
    reset_all();
    let chip = PwmChip::new(0);
    let channel = chip.get_channel(0);
    channel.set_period(Duration::from_micros(50));
    channel.set_duty_cycle(Duration::from_micros(25));
    channel.enable();    
}
```