use std::{cell::RefCell, path::Path, rc::Rc};

use mygbcartridge::cartridge::Cartridge;

use crate::{PPU, cpu::cpu::CPU};

use super::mem_map::MemMap;

pub(crate) struct Device {
    pub(crate) ppu: PPU,
    pub(crate) cpu: CPU,
    pub(crate) io: Rc<RefCell<MemMap>>,
}

impl Device {
    pub fn new() -> Device {
        let io = Rc::new(RefCell::new(MemMap::new()));
        let cpu = CPU::new(Rc::clone(&io));
        let ppu = PPU::new(Rc::clone(&io));
        Device { cpu, ppu, io }
    }

    pub fn load_path(&mut self, path: &Path) {
        let cartridge = Cartridge::new(path);
        self.io.borrow_mut().set_cartridge(&cartridge);
        self.run();
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.execute();
        }
    }
}
