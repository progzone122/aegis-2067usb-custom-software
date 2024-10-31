mod led;

use anyhow::{Result, Context, anyhow};
use futures_lite::future::block_on;
use nusb;
use nusb::transfer::{Control, ControlIn, ControlType, Recipient};

pub struct Api {
    pub vid: u16,
    pub pid: u16
}
impl Default for Api {
    fn default() -> Self {
        Self {
            vid: 0x1A2C,
            pid: 0x9CF4
        }
    }
}

struct Command {
    bm_request_type: u8,
    bm_request: u8,
    w_value: u16,
    w_index: u16,
    data: Vec<u8>
}
impl Command {
    pub fn new(bm_request_type: u8, bm_request: u8, w_value: u16, w_index: u16, data: Vec<u8>) -> Self {
        Command {
            bm_request_type,
            bm_request,
            w_value,
            w_index,
            data,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(self.bm_request_type);
        bytes.push(self.bm_request);
        bytes.extend_from_slice(&self.w_value.to_le_bytes()); // Little-endian
        bytes.extend_from_slice(&self.w_index.to_le_bytes()); // Little-endian
        bytes.extend_from_slice(&self.data);

        bytes
    }
}

impl Api {
    pub fn new(vid: u16, pid: u16) -> Self {
        Self {
            vid,
            pid
        }
    }
    fn get_device(&self) -> Result<nusb::DeviceInfo> {
        let devices_list = nusb::list_devices()
            .context("Failed to list USB devices")?;

        devices_list
            .into_iter()
            .find(|dev| dev.vendor_id() == self.vid && dev.product_id() == self.pid)
            .with_context(|| format!("Device with VID {:04X} and PID {:04X} not found", self.vid, self.pid))
    }
    pub fn connect_device(&self, interface: u8) -> Result<nusb::Interface> {
        let device: nusb::DeviceInfo = self.get_device()?;

        println!("Connected device {:#?}", device.interfaces().collect::<Vec<_>>());

        let connection: nusb::Device = device
            .open()
            .context(format!("Failed to open USB device (VID: {:04X}, PID: {:04X}", self.vid, self.pid))?;

        let interface: nusb::Interface = connection.detach_and_claim_interface(interface)?;

        interface.set_alt_setting(0)?;

        interface.control_out_blocking(Control {
            control_type: ControlType::Standard,
            recipient: Recipient::Interface,
            request: 0x09,
            value: 0x0307,
            index: 0,
        }, &mut [0x07, 0xff, 0xff, 0x07, 0x05, 0x01, 0x00, 0x00], Default::default())?;

        // interface.clear_halt(0x03)?;

        // let endpoint =

        // interface.

        // let buf = Command::new(
        //     0x21,
        //     0x09,
        //     0x0307,
        //     1,
        //     vec![0x07, 0xFF, 0xFF, 0x07, 0x05, 0x01, 0x00, 0x00]
        // );
        //
        // interface.interrupt_out(0x01, buf.to_bytes());


        Ok(interface)
    }
}