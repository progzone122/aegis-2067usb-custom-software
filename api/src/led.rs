use nusb::transfer::{Control, ControlType, Recipient};
use anyhow::{Result, Context, anyhow};
use crate::values::AnimationEffect;

pub struct LED {
    animation: u8,
    speed: u8,
    brightness: u8,
}

impl Default for LED {
    fn default() -> Self {
        Self {
            animation: 0x00,
            speed: 0x02,
            brightness: 0x05,
        }
    }
}
impl LED {
    fn build_command(&self, interface: nusb::Interface, animation_effect: u8, brightness: u8, speed: u8) -> Result<usize> {
        let result: usize = interface.control_out_blocking(Control {
            control_type: ControlType::Class,
            recipient: Recipient::Interface,
            request: 0x09,
            value: 0x0207,
            index: 0,
        }, &mut [0x07, 0xff, 0xff, animation_effect, brightness, speed, 0x00, 0x00], Default::default())?;

        Ok(result)
    }
    pub fn change_animation_effect(&mut self, interface: nusb::Interface, animation_effect: u8) -> Result<usize> {
        self.animation = animation_effect;

        Ok(self.build_command(interface, self.animation, self.brightness, self.speed)?)
    }
    pub fn change_brightness(&mut self, interface: nusb::Interface, brightness: u8) -> Result<usize> {
        if brightness > 5 {
            Err(anyhow!("Invalid brightness value: must be between 0 and 5"))?
        }
        self.brightness = brightness;

        Ok(self.build_command(interface, self.animation, self.brightness, self.speed)?)
    }
}

