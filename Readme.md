# Oxy-linux-plus
This is a Rust PWM crate for Linux that automatically selects software or hardware implementation. Hardware PWM will be implemented through the Linux sysfs ABI. Please check if your PWM chip is under /sys/class/pwm directory.
(The software pwm engine has not been done)