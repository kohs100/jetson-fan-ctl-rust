# jetson-fan-ctl-rust
Automated fan control for Jetson nano implemented in Rust.

## Requirements:

### Hardware
You will need a 5V PWM fan for this to make any sense.  
I used the **Noctua nf-a4x20 5V PWM** fan.

Additionally, I recommend you use the barrel jack with a 4A power supply.  

### Software
I will assume you use the standard image on your jetson nano.

In other words, there must be following devices and valid.
* /sys/devices/pwm-fan/target_pwm
* /sys/devices/virtual/thermal/thermal_zone0
* /sys/devices/virtual/thermal/thermal_zone1
* /sys/devices/virtual/thermal/thermal_zone2
* /sys/devices/virtual/thermal/thermal_zone3
* /sys/devices/virtual/thermal/thermal_zone4
* /sys/devices/virtual/thermal/thermal_zone5

GLIBC 2.17 or later version is needed. 
Tested and build in Jetson nano Rev.A02 w/ L4T 32.7.3 (Linux 4.9.299-tegra), which is the latest official L4T image supports Jetson nano.

## How to install:
<code>$ sudo ./install.sh</code>

The script will automatically run at boot time.
It's a set-it-and-forget-it type thing, unless you want to mess with the fan speeds.

## How to customize:
open /etc/automagic-fan/config.json with your favorite editor (I'm using nano):  

<code>$ sudo nano /etc/jetson-fan-ctl-rust/config.json</code>

you will find the following lines:
```
    {
        "min_temp": 30000,
        "max_temp": 60000,
        "interval_sec": 5,
        "thermal_zone": "FAN",
        "max_perf": true
    }
```
<code>min_temp</code> is the temperature (0.001°C) below which the fan is at 0% speed.  
<code>max_temp</code> is the temperature (0.001°C) above which the fan is at 100% speed.  
The script interpolates linearly between these two points.

<code>thermal_zone</code> is the reference virtual thermal zone provided by Jetson. You can set it from the list below.
* A0
* CPU
* GPU
* PLL
* PMIC
* FAN

<code>interval_sec</code> tells the script how often to update the fan speed (in seconds).  
<code>max_perf</code> will execute the jetson_clocks script at sevice start if set.

You can only use integers in each of these fields. No floating point values allowed.
The temperature precision of the thermal sensors is 0.5 (°C), so don't expect this to be too precise.

Any changes in the script will be will be applied after the next reboot.  
You can run

    sudo service jetson-fan-ctl-rust restart

to apply changes immediately.

If you suspect something went wrong, please check:

    sudo service jetson-fan-ctl-rust status

## Credit
This project is basically the Rust-ported version of the project below.
(https://github.com/Pyrestone/jetson-fan-ctl)