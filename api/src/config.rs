use std::fs;
use std::path::{Path, PathBuf};
use ini::{Ini, Properties};
use dirs::config_dir;

#[derive(Debug)]
pub struct Config {
    pub animation: u8,
    pub speed: u8,
    pub brightness: u8,
    config: Ini,
    path: PathBuf
}

impl Config {
    pub fn load() -> Self {
        let mut path: PathBuf = config_dir().expect("Could not determine config directory");
        path.push("aegis-2067usb");

        fs::create_dir_all(&path).expect("Failed to create config directory");

        path.push("config.conf");

        let config: Ini = if path.exists() {
            Ini::load_from_file(&path).expect("Failed to load configuration file")
        } else {
            let mut new_config: Ini = Ini::new();
            new_config.with_section(Some("led"))
                .set("animation", "0x00")
                .set("speed", "0x02")
                .set("brightness", "0x05");

            new_config.write_to_file(&path).expect("Failed to create configuration file");
            new_config
        };

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

        section.insert("animation", format!("0x{:02X}", self.animation));
        section.insert("speed", format!("0x{:02X}", self.speed));
        section.insert("brightness", format!("0x{:02X}", self.brightness));

        self.config.write_to_file(&self.path).expect("Failed to save configuration file");
    }
}