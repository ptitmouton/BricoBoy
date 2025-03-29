use std::{path::Path, thread};

use mygbcartridge::cartridge::Cartridge;

use crate::{
    PPU,
    cpu::cpu::{CPU, M_CYCLE_LENGTH, T_CYCLE_LENGTH},
};

use super::mem_map::MemMap;

pub(crate) struct Device {
    pub(crate) ppu: PPU,
    pub(crate) cpu: CPU,
    pub(crate) mem_map: MemMap,

    pub running: bool,
}

impl Device {
    pub fn new() -> Device {
        let mem_map = MemMap::new();
        let cpu = CPU::new();
        let ppu = PPU::new();
        let running = false;
        println!("Device created");
        Device {
            cpu,
            ppu,
            mem_map,
            running,
        }
    }

    pub fn load_path(&mut self, path: &Path) {
        let cartridge = Cartridge::new(path);
        self.mem_map.set_cartridge(&cartridge);
    }

    pub fn run(&mut self) {
        let _ = self.run_loop();
    }

    pub(crate) fn ppu_enabled(&self) -> bool {
        self.mem_map.io_registers.get_lcdl_register().lcd_enabled()
    }

    fn run_loop(&mut self) {
        loop {
            if !self.running {
                break;
            }
            self.cycle();
        }
    }

    pub fn cycle(&mut self) {
        // TODO: Let the CPU and PPU run in parallel
        // TODO: seems like UI is unable to update while we are running the device
        thread::scope(|s| {
            s.spawn(|| {
                let cycle_counts = self.cpu.execute(&mut self.mem_map);
                if self.ppu_enabled() {
                    // ppu cycles in t_cycles, of which there are 4 per m_cycle
                    for _ in 0..(cycle_counts * 4) {
                        self.ppu.cycle(&mut self.mem_map);
                    }
                }
                let sleep_dur = M_CYCLE_LENGTH * cycle_counts;
                println!("Sleeping for {:?}", sleep_dur);
                thread::sleep(sleep_dur);
            });
        });
    }
}
