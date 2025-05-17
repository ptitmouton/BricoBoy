use super::generic_memory::{GenericMemory, OffsetMemory, ReadableMemory, WritableMemory};

pub struct HRAM {
    pub data: [u8; 127],
}

impl OffsetMemory for HRAM {
    fn offset(&self) -> u16 {
        0xff80
    }
}

impl GenericMemory<127> for HRAM {
    fn read_byte(&self, address: u16) -> u8 {
        self.data.read_byte(self.map_address(address))
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.data.write_byte(self.map_address(address), value)
    }

    fn read_word(&self, address: u16) -> u16 {
        self.data.read_word(self.map_address(address))
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.data.write_word(self.map_address(address), value)
    }
}

impl HRAM {
    pub fn new() -> HRAM {
        let data = [0; 127];

        HRAM { data }
    }
}
