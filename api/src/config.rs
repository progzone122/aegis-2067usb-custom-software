use std::path::Path;
use ini::{Ini, Properties};

#[derive(Debug)]
pub struct Config {
    pub animation: u8,
    pub speed: u8,
    pub brightness: u8,
    config: Ini,
    path: String
}

impl Config {
    pub fn load() -> Self {
        let path: String = "~/.config/aegis-2067usb/config.conf".replace("~", &std::env::var("HOME").unwrap());

        let config = Ini::load_from_file(Path::new(&path)).expect("Failed to load configuration file");
        let section = config.section(Some("led")).expect("Failed to find section [led]");

        let animation: u8 = section.get("animation")
            .unwrap_or("0x00")
            .trim_start_matches("0x0")
            .parse::<u8>()
            .unwrap_or(0x00);

        let speed: u8 = section.get("speed")
            .unwrap_or("0x02")
            .trim_start_matches("0x0")
            .parse::<u8>()
            .unwrap_or(0x02);

        let brightness: u8 = section.get("brightness")
            .unwrap_or("0x05")
            .trim_start_matches("0x0")
            .parse::<u8>()
            .unwrap_or(0x05);

        Self {
            animation,
            speed,
            brightness,
            config,
            path
        }
    }

    pub fn set_animation(&mut self, animation: u8) {
        self.animation = animation;
        self.save();
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed;
        self.save();
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness;
        self.save();
    }

    fn save(&mut self) {
        let section: &mut Properties = self.config.section_mut(Some("led")).expect("Failed to find section [led]");

        println!("{}", format!("0x{:02X}", self.animation));
        println!("{}", format!("0x{:02X}", self.speed));
        println!("{}", format!("0x{:02X}", self.brightness));

        section.insert("animation", self.animation.to_string());
        section.insert("speed", format!("0x{:02X}", self.speed));
        section.insert("brightness", format!("0x{:02X}", self.brightness));

        self.config.write_to_file(Path::new(&self.path)).expect("Failed to save configuration file");
    }
}