# Oxy-linux-pwm
这是一个linux下的rust pwm crate，它将自动选择软件或硬件实现，硬件pwm将通过linux sysfs abi是实现。请检查/sys/class/pwm目录下是否有你的pwm芯片
(软件实现暂未完成)