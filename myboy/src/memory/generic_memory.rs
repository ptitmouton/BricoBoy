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
        (address as usize) - self.offset()
    }
}

impl OffsetMemory for GenericRam {
    fn offset(&self) -> usize {
        self.offset
    }
}

impl GenericRam {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[self.map_address(address)]
    }

    pub fn create(size: usize, offset: usize) -> GenericRam {
        GenericRam {
            data: vec![0; size],
            offset,
            size,
        }
    }
}

impl ReadableMemory for GenericRam {
    fn read_byte(&self, address: u16) -> u8 {
        let mapped_address = self.map_address(address);
        self.data[mapped_address]
    }

    fn read_word(&self, address: u16) -> u16 {
        let mapped_address = self.map_address(address) as u16;
        let results = *self.read_fixed_bytes(mapped_address, 2);
        return u16::from_le_bytes(results);
    }

    fn read_fixed_bytes<const L: usize>(&self, address: u16, length: usize) -> &[u8; L] {
        let mapped_address = self.map_address(address);
        self.data[mapped_address..mapped_address + length]
            .try_into()
            .unwrap()
    }
}

impl WritableMemory for GenericRam {
    fn write_byte(&mut self, address: u16, value: u8) {
        let mapped_address = self.map_address(address);
        self.data[mapped_address] = value;
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let mapped_address = self.map_address(address) as u16;
        self.write_bytes(mapped_address, &value.to_le_bytes());
    }

    fn write_bytes(&mut self, address: u16, value: &[u8]) {
        let mapped_address = self.map_address(address) as u16;
        for (i, byte) in value.iter().enumerate() {
            self.write_byte(mapped_address + i as u16, *byte);
        }
    }
}

pub struct GenericRam {
    data: Vec<u8>,
    offset: usize,
    size: usize,
}
