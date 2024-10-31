pub mod led;
pub mod values;

use anyhow::{Result, Context, anyhow};
use nusb;

pub struct Api {
    pub vid: u16,
    pub pid: u16,
    pub interface: Option<nusb::Interface>
}
impl Default for Api {
    fn default() -> Self {
        Self {
            vid: 0x1A2C,
            pid: 0x9CF4,
            interface: None
        }
    }
}

impl Api {
    pub fn new(vid: u16, pid: u16) -> Self {
        Self {
            vid,
            pid,
            interface: None
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
}