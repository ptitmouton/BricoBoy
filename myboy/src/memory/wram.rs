use super::generic_memory::{GenericMemory, OffsetMemory, ReadableMemory, WritableMemory};

pub struct WRAM {
    pub data: [u8; 4096],
}

impl OffsetMemory for WRAM {
    fn offset(&self) -> u16 {
        0x8000
    }
}

impl GenericMemory<8192> for WRAM {
    fn read_byte(&self, address: u16) -> u8 {
        self.data.read_byte(self.map_address(address))
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.data.write_byte(address, value)
    }

    fn read_word(&self, address: u16) -> u16 {
        self.data.read_word(self.map_address(address))
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.data.write_word(address, value)
    }
}

impl WRAM {
    pub fn new() -> WRAM {
        let data = [0; 4096];

        WRAM { data }
    }
}
