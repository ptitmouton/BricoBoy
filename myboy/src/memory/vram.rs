use super::generic_memory::{GenericMemory, OffsetMemory, ReadableMemory, WritableMemory};

pub struct VRAM {
    pub data: [u8; 8192],
}

impl OffsetMemory for VRAM {
    fn offset(&self) -> u16 {
        0x8000
    }
}

impl GenericMemory<8192> for VRAM {
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

impl VRAM {
    pub fn new() -> VRAM {
        let data = [0; 8192];

        VRAM { data }
    }
}
