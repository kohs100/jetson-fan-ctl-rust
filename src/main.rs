use configparser::ini::Ini;
use std::{fs, process, thread, time};

#[derive(Copy, Clone, Debug)]
enum ThermalZone {
    A0 = 0,
    CPU = 1,
    GPU = 2,
    PLL = 3,
    PMIC = 4,
    FAN = 5,
}

struct Config {
    min_temp: i64,
    max_temp: i64,
    interval_sec: u64,
    thermal_zone: ThermalZone,
    max_perf: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            min_temp: 30,
            max_temp: 60,
            interval_sec: 5,
            thermal_zone: ThermalZone::FAN,
            max_perf: true,
        }
    }
}

impl Config {
    fn read_temp(&self) -> i64 {
        let zone = self.thermal_zone as u8;
        let path = format!("/sys/devices/virtual/thermal/thermal_zone{}/temp", zone);
        let s_temp = fs::read_to_string(path).expect("Could not read temperature device");
        s_temp
            .trim()
            .parse::<i64>()
            .expect("Invalid temperature value detected")
    }

    fn get_interval(&self) -> time::Duration {
        time::Duration::from_secs(self.interval_sec)
    }

    fn get_fancurve(&self) -> u8 {
        let temp = self.read_temp();
        if temp < self.min_temp {
            0u8
        } else if temp < self.max_temp {
            let ot = temp - self.min_temp;
            let fan = (ot * 255) / (self.max_temp - self.min_temp);
            fan.max(0).min(255) as u8
        } else {
            255u8
        }
    }

    fn set_perf(&self) {
        if self.max_perf {
            process::Command::new("/usr/bin/jetson_clocks")
                .output()
                .expect("Running /usr/bin/jetson_clocks failed");
        }
    }

    fn read_conf() -> Self {
        let mut conf = Ini::new();
        conf.load("/etc/jetson-fan-ctl-rust/config.ini")
            .expect("Failed to read config.ini file");

        let mut config = Config::default();

        if let Some(i) = conf.get("Configuration", "min_temp") {
            config.min_temp = i.parse::<i64>().expect("Invalid min_temp value");
        } else {
            println!(
                "No min_temp specified in config. Falling to default {}...",
                config.min_temp
            );
        }
        if let Some(i) = conf.get("Configuration", "max_temp") {
            config.max_temp = i.parse::<i64>().expect("Invalid max_temp value");
        } else {
            println!(
                "No max_temp specified in config. Falling to default {}...",
                config.max_temp
            );
        }
        if let Some(i) = conf.get("Configuration", "interval_sec") {
            config.interval_sec = i.parse::<u64>().expect("Invalid interval_sec value");
        } else {
            println!(
                "No interval_sec specified in config. Falling to default {}...",
                config.interval_sec
            );
        }
        if let Some(i) = conf.get("Configuration", "thermal_zone") {
            config.thermal_zone = match i.as_str() {
                "A0" => ThermalZone::A0,
                "CPU" => ThermalZone::CPU,
                "GPU" => ThermalZone::GPU,
                "PLL" => ThermalZone::PLL,
                "PMIC" => ThermalZone::PMIC,
                "FAN" => ThermalZone::FAN,
                _ => panic!("Invalid zone value"),
            };
        } else {
            println!(
                "No thermal_zone specified in config. Falling to default {:?}...",
                config.thermal_zone
            );
        }
        if let Ok(Some(i)) = conf.getbool("Configuration", "max_perf") {
            config.max_perf = i;
        } else {
            println!(
                "No max_perf specified in config. Falling to default {}...",
                config.max_perf
            );
        }
        assert!(
            config.min_temp < config.max_temp,
            "max temp is lower them min temp. Terminating..."
        );

        config
    }
}

struct FanState {
    cur_speed: u8,
}

impl FanState {
    fn set(&mut self, speed: u8) {
        if speed != self.cur_speed {
            fs::write("/sys/devices/pwm-fan/target_pwm", speed.to_string())
                .expect("Coult not write to pwm-fan device");
        }
    }

    fn new() -> Self {
        let mut new = Self { cur_speed: 255 };
        new.set(0);
        new
    }
}

fn main() {
    let conf = Config::read_conf();
    conf.set_perf();
    let time_dur = conf.get_interval();
    let mut fan_state = FanState::new();

    println!("Hello, Jetson!");

    loop {
        let speed = conf.get_fancurve();
        fan_state.set(speed);
        thread::sleep(time_dur);
    }
}
