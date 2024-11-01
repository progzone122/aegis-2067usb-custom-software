use anyhow::{Result, Context, anyhow};
use crate::config::Config;
use crate::Api;

pub mod led {
    use super::*;

    pub fn change_animation_effect(config: &mut Config, interface: nusb::Interface, animation_effect: u8) -> Result<usize> {
        config.set_animation(animation_effect);

        Ok(Api::build_command(interface, config.animation, config.brightness, config.speed)?)
    }
    pub fn change_speed(config: &mut Config, interface: nusb::Interface, speed: u8) -> Result<usize> {
        if speed > 2 {
            Err(anyhow!("Invalid brightness value: must be between 0 and 2"))?
        }
        config.set_speed(speed);

        Ok(Api::build_command(interface, config.animation, config.brightness, speed)?)
    }
    pub fn change_brightness(config: &mut Config, interface: nusb::Interface, brightness: u8) -> Result<usize> {
        if brightness > 5 {
            Err(anyhow!("Invalid brightness value: must be between 0 and 5"))?
        }
        config.set_brightness(brightness);

        Ok(Api::build_command(interface, config.animation, brightness, config.speed)?)
    }
}