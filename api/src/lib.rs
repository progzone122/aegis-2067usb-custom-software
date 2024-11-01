pub mod led;
pub mod values;
pub mod config;
pub mod other;

use anyhow::{Result, Context};
use nusb;
use nusb::transfer::{Control, ControlType, Recipient};

pub struct Api {
    pub vid: u16,
    pub pid: u16
}
impl Default for Api {
    fn default() -> Self {
        Self {
            vid: 0x1A2C,
            pid: 0x9CF4,
        }
    }
}

impl Api {
    pub fn new(vid: u16, pid: u16) -> Self {
        Self {
            vid,
            pid,
        }
    }
    fn get_device(&self) -> Result<nusb::DeviceInfo> {
        let devices_list = nusb::list_devices()
            .context("Failed to list USB devices")?;

        devices_list
            .into_iter()
            .find(|dev| dev.vendor_id() == self.vid && dev.product_id() == self.pid)
            .context(format!("Device with VID {:04X} and PID {:04X} not found", self.vid, self.pid))
    }
    pub fn connect_device(&self, interface: u8) -> Result<nusb::Interface> {
        let device: nusb::DeviceInfo = self.get_device()?;

        let connection: nusb::Device = device
            .open()
            .context(format!("Failed to open USB device (VID: {:04X}, PID: {:04X}", self.vid, self.pid))?;

        let interface: nusb::Interface = connection.detach_and_claim_interface(interface)?;

        Ok(interface)
    }
    pub fn build_command(interface: nusb::Interface, animation_effect: u8, brightness: u8, speed: u8) -> Result<usize> {
        let result: usize = interface.control_out_blocking(Control {
            control_type: ControlType::Class,
            recipient: Recipient::Interface,
            request: 0x09,
            value: 0x0207,
            index: 0,
        }, &mut [0x07, 0xff, 0xff, animation_effect, brightness, speed, 0x00, 0x00], Default::default())?;

        Ok(result)
    }
}