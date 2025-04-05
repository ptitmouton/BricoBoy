use std::{thread, time::Instant};

use mygbcartridge::cartridge::Cartridge;

use crate::{
    PPU,
    cpu::cpu::{CPU, CYCLE_LENGTH},
};

use super::mem_map::MemMap;

pub(crate) struct Device {
    pub ppu: PPU,
    pub cpu: CPU,
    pub mem_map: MemMap,

    pub speed_multiplier: f32,

    pub running: bool,

    pub serial_buffer: Vec<u8>,

    pub breakpoint: Option<u16>,
}

impl Device {
    pub fn new(cartridge: &Cartridge) -> Device {
        let mem_map = MemMap::new(cartridge.clone());
        let cpu = CPU::new();
        let ppu = PPU::new();
        let running = false;
        let serial_buffer = Vec::new();

        Device {
            cpu,
            ppu,
            speed_multiplier: 1.0,
            mem_map,
            running,
            serial_buffer,
            breakpoint: Some(0x02a0),
        }
    }

    pub(crate) fn toggle_breakpoint(&mut self, addr: u16) {
        match self.breakpoint {
            Some(breakpoint) if breakpoint == addr => {
                self.breakpoint = None;
            }
            _ => {
                self.breakpoint = Some(addr);
            }
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        let _ = self.run_loop();
    }

    pub(crate) fn ppu_enabled(&self) -> bool {
        self.mem_map.io_registers.get_lcdl_register().lcd_enabled()
    }

    fn run_loop<'a>(&'a mut self) {
        loop {
            if let Some(addr) = self.breakpoint {
                if *self.cpu.register_set.pc() == addr {
                    println!("Breakpoint hit at 0x{:04x}", self.breakpoint.unwrap());
                    self.running = false;
                }
            }
            if !self.running {
                break;
            }
            self.step();
        }
    }

    pub fn step(&mut self) {
        loop {
            self.cycle();
            self.cycle();
            self.cycle();
            self.cycle();

            if !self.cpu.is_busy() {
                break;
            }
        }

        self.check_serial();
    }

    fn cycle<'a>(&'a mut self) {
        let speed_multiplier = self.speed_multiplier;
        unsafe {
            let cycle_start = Instant::now();
            // TODO: Maybe there's a more elegant way?
            let raw_device_pointer = self as *mut Device as usize;
            {
                let raw_device = raw_device_pointer as *mut Device;
                let device = &mut *raw_device;
                device.cpu.cycle(&mut device.mem_map);
            }
            {
                let raw_device = raw_device_pointer as *mut Device;
                let device = &mut *raw_device;
                if self.ppu_enabled() {
                    device.ppu.cycle(&mut device.mem_map);
                }
            }
            let cycle_duration = cycle_start.elapsed();
            println!(
                "cycle length: {:?} - gb cycle length: {:?}",
                cycle_duration, CYCLE_LENGTH
            );
            let cycle_rest = CYCLE_LENGTH.checked_sub(cycle_duration).unwrap_or_default();
            if cycle_rest.as_nanos() > 0 {
                let sleep_dur = cycle_rest.div_f32(speed_multiplier);
                // println!("Sleeping for {:?} ({:?} realtime)", sleep_dur, cycle_rest);
                thread::sleep(sleep_dur);
            }
        }
    }

    fn check_serial(&mut self) {
        println!(
            "Checking serial: {:0x}",
            self.mem_map.io_registers.read_byte(0xff02)
        );
        if self.mem_map.io_registers.read_byte(0xff02) == 0x81 {
            let data = self.mem_map.io_registers.read_byte(0xff01);
            self.serial_buffer.push(data);
            println!("Serial data: {:?}", data);
            self.mem_map.io_registers.write_byte(0xff02, 0x00);
        }
    }
}
