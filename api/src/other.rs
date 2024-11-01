use anyhow::{Result, Context, anyhow};
use crate::config::Config;
use crate::Api;

pub mod other {
    use super::*;

    pub fn reset_settings(config: &mut Config, interface: nusb::Interface) -> Result<usize> {
        config.set_animation(0x00);
        config.set_speed(0x02);
        config.set_brightness(0x05);

        Ok(Api::build_command(interface, config.animation, config.brightness, config.speed)?)
    }
}