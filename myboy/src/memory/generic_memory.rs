pub trait ReadableMemory {
    fn read_byte(&self, address: u16) -> u8;
    fn read_fixed_bytes<const L: usize>(&self, address: u16, length: usize) -> &[u8; L];
    fn read_word(&self, address: u16) -> u16;
}

pub trait WritableMemory {
    fn write_byte(&mut self, address: u16, value: u8);
    fn write_bytes(&mut self, address: u16, value: &[u8]);
    fn write_word(&mut self, address: u16, value: u16);
}

pub trait OffsetMemory {
    fn offset(&self) -> usize;
    fn map_address(&self, address: u16) -> usize {
        println!(
            "Mapping address: 0x{:04x} to offset: 0x{:04x}",
            address,
            self.offset()
        );
        (address as usize) - self.offset()
    }
}

impl OffsetMemory for RWMemory {
    fn offset(&self) -> usize {
        self.offset
    }
}

impl RWMemory {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[self.map_address(address)]
    }

    pub fn create(size: usize, offset: usize) -> RWMemory {
        RWMemory {
            data: vec![0; size],
            offset,
            size,
        }
    }
}

impl ReadableMemory for RWMemory {
    fn read_byte(&self, address: u16) -> u8 {
        let mapped_address = self.map_address(address);
        self.data[mapped_address]
    }

    fn read_word(&self, address: u16) -> u16 {
        let results = *self.read_fixed_bytes(address, 2);
        return u16::from_le_bytes(results);
    }

    fn read_fixed_bytes<const L: usize>(&self, address: u16, length: usize) -> &[u8; L] {
        let mapped_address = self.map_address(address);
        self.data[mapped_address..mapped_address + length]
            .try_into()
            .unwrap()
    }
}

impl WritableMemory for RWMemory {
    fn write_byte(&mut self, address: u16, value: u8) {
        let mapped_address = self.map_address(address);
        self.data[mapped_address] = value;
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let mapped_address = self.map_address(address);
        let values = value.to_le_bytes();
        self.data[mapped_address] = values[0];
        self.data[mapped_address + 1] = values[1];
    }

    fn write_bytes(&mut self, address: u16, value: &[u8]) {
        let mapped_address = self.map_address(address) as u16;
        for (i, byte) in value.iter().enumerate() {
            self.write_byte(mapped_address + i as u16, *byte);
        }
    }
}

pub struct RWMemory {
    data: Vec<u8>,
    offset: usize,
    size: usize,
}
