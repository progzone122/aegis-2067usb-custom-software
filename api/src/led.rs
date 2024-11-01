use nusb::transfer::{Control, ControlType, Recipient};
use anyhow::{Result, Context, anyhow};
use crate::config::Config;

pub struct LED;

impl LED {
    fn build_command(interface: nusb::Interface, animation_effect: u8, brightness: u8, speed: u8) -> Result<usize> {
        let result: usize = interface.control_out_blocking(Control {
            control_type: ControlType::Class,
            recipient: Recipient::Interface,
            request: 0x09,
            value: 0x0207,
            index: 0,
        }, &mut [0x07, 0xff, 0xff, animation_effect, brightness, speed, 0x00, 0x00], Default::default())?;

        Ok(result)
    }
    pub fn change_animation_effect(config: &mut Config, interface: nusb::Interface, animation_effect: u8) -> Result<usize> {
        config.set_animation(animation_effect);

        Ok(LED::build_command(interface, config.animation, config.brightness, config.speed)?)
    }
    pub fn change_speed(config: &mut Config, interface: nusb::Interface, speed: u8) -> Result<usize> {
        if speed > 2 {
            Err(anyhow!("Invalid brightness value: must be between 0 and 2"))?
        }
        config.set_speed(speed);

        Ok(LED::build_command(interface, config.animation, config.brightness, speed)?)
    }
    pub fn change_brightness(config: &mut Config, interface: nusb::Interface, brightness: u8) -> Result<usize> {
        if brightness > 5 {
            Err(anyhow!("Invalid brightness value: must be between 0 and 5"))?
        }
        config.set_brightness(brightness);

        Ok(LED::build_command(interface, config.animation, brightness, config.speed)?)
    }
}