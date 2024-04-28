# Oxy-linux-pwm
这是一个linux下的rust pwm crate，它将自动选择软件或硬件实现，硬件pwm将通过linux sysfs abi是实现。请检查/sys/class/pwm目录下是否有你的pwm芯片
(软件实现暂未完成)

``` rust
// channel被释放时，对应通道会自动关闭和取消导出
fn demo() {
    reset_all();
    let chip = PwmChip::new(0);
    let channel = chip.get_channel(0);
    channel.set_period(Duration::from_micros(50));
    channel.set_duty_cycle(Duration::from_micros(25));
    channel.enable();    
}
```