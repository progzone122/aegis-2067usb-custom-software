struct LED {
    value: u16,
    index: u8,
    length: u8
}

impl Default for LED {
    fn default() -> Self {
        Self {
            value: 0x0307,
            index: 1,
            length: 8,
        }
    }
}


