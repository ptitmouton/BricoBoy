use crate::{
    cpu::CYCLE_LENGTH,
    logging::log::{ConsoleLogger, Log, Logger},
};
use std::{thread, time::Instant};

use mygbcartridge::cartridge::Cartridge;

use crate::{PPU, cpu::CPU};

use super::mem_map::MemMap;

pub(crate) struct Device<'a> {
    pub ppu: PPU<'a>,
    pub cpu: CPU,
    pub mem_map: MemMap,
    pub screen: Box<[u8]>,

    pub speed_multiplier: f64,

    pub running: bool,

    pub serial_buffer: Vec<u8>,

    pub breakpoint: Option<u16>,

    pub cartridge: Cartridge,

    pub logger: Box<dyn Logger>,
}

impl<'a> Device<'a> {
    pub fn new(cartridge: Cartridge) -> Device<'a> {
        let mem_map = MemMap::new(cartridge.clone());
        let cpu = CPU::new();
        let ppu = PPU::new();

        let screen = Box::new([0xff as u8; 160 * 144 * 4]);

        let running = false;
        let serial_buffer = Vec::new();
        let logger = Box::new(ConsoleLogger::default());

        Device {
            cpu,
            ppu,

            screen,

            cartridge,
            speed_multiplier: 1.0,
            mem_map,
            running,
            serial_buffer,
            breakpoint: None,
            logger,
            // breakpoint: Some(0xcb23),
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
        self.mem_map.io_registers.get_lcdc_register().lcd_enabled()
    }

    fn run_loop(&mut self) {
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

    fn cycle(&mut self) {
        let speed_multiplier = self.speed_multiplier;
        unsafe {
            let cycle_start = Instant::now();
            // TODO: Maybe there's a more elegant way?
            let raw_device_pointer = self as *mut Device as usize;
            {
                let raw_device = raw_device_pointer as *mut Device;
                let device = &mut *raw_device;
                device.cpu.cycle(&mut device.mem_map, self.logger.as_mut());
            }
            {
                let raw_device = raw_device_pointer as *mut Device;
                let device = &mut *raw_device;
                if self.ppu_enabled() {
                    device.ppu.cycle(&mut device.mem_map, self.screen.as_mut());
                }
            }
            let cycle_duration = cycle_start.elapsed();
            let cycle_rest = CYCLE_LENGTH.checked_sub(cycle_duration).unwrap_or_default();
            if cycle_rest.as_nanos() > 0 {
                let sleep_dur = cycle_rest.div_f64(speed_multiplier);
                thread::sleep(sleep_dur);
            }
        }
    }

    pub fn draw_screen(&mut self, pixels: &mut [u8]) {
        for i in 0..(160 * 144 * 4) {
            pixels[i] = self.screen[i];
        }
    }

    fn check_serial(&mut self) {
        if self.mem_map.io_registers.read_byte(0xff02) == 0x81 {
            let data = self.mem_map.io_registers.read_byte(0xff01).clone();
            self.serial_buffer.push(data);
            self.mem_map.io_registers.write_byte(0xff02, 0x00);

            self.log_serial_output(data as char);
        }
    }

    fn log_serial_output(&mut self, data: char) {
        self.logger.info(Log::SerialOutput(data))
    }
}
