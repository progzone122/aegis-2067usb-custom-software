use nusb;
struct Api {
    vid: i32,
    pid: i32
}
impl Default for Api {
    fn default() -> Self {
        Self {
            vid: 0x1A2C,
            pid: 0x9CF4
        }
    }
}
impl Api {
    pub fn new(vid: i32, pid: i32) -> Self {
        Self {
            vid,
            pid
        }
    }
    fn get_device() {
        // let device: nusb::list_devices().unwrap()

    }
}