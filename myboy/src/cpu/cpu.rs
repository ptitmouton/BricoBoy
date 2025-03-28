use super::{instruction::Instruction, register_set::RegisterSet};
use crate::{cpu::register_set::Flag, memory::memory_map::MemoryMap};
use std::{fmt::Display, num::Wrapping, ops::Add, thread::sleep, time::Duration};

const SPEED: u64 = 8;
const CPU_FREQUENCY: u64 = 4_194_304 * SPEED;
const M_CYCLE_COUNT: u64 = CPU_FREQUENCY / 4;

const M_CYCLE_LENGTH: Duration = Duration::from_nanos(1_000 / M_CYCLE_COUNT);

pub enum InterruptMasterEnableStatus {
    Enabled,
    Enabling,
    Disabled,
}

pub struct CPU<'a> {
    pub memory_map: &'a mut MemoryMap,
    pub register_set: RegisterSet,
    pub(super) interrupt_master_enable: InterruptMasterEnableStatus,

    running: bool,
    stepping: bool,
    cycle_count: Wrapping<u64>,
}

impl CPU<'_> {
    pub fn new<'a>(memory_map: &'a mut MemoryMap) -> CPU<'a> {
        let register_set = RegisterSet::default();

        CPU {
            memory_map,
            register_set,
            interrupt_master_enable: InterruptMasterEnableStatus::Disabled,
            cycle_count: Wrapping(0),

            running: true,
            stepping: false,
        }
    }

    pub fn execute(&mut self) {
        self.check_interrupts();

        let next_instruction_address = self.register_set.pc();
        let instruction = Instruction::new(
            self.memory_map.read_byte(next_instruction_address),
            next_instruction_address,
        );
        println!("Next op: {}", instruction);
        let cycles_past = self.run(&instruction);
        self.cycle_count.add(Wrapping(cycles_past as u64));
        sleep(M_CYCLE_LENGTH.saturating_mul(cycles_past));
    }

    fn check_interrupts(&mut self) {
        match self.interrupt_master_enable {
            InterruptMasterEnableStatus::Enabled => {
                // Check for interrupts
            }
            InterruptMasterEnableStatus::Enabling => {
                self.interrupt_master_enable = InterruptMasterEnableStatus::Enabled
            }
            InterruptMasterEnableStatus::Disabled => {}
        }
    }
}

impl Display for CPU<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut flags = String::new();
        if self.register_set.get_flag(Flag::Zero) {
            flags.push_str(" Z");
        }
        if self.register_set.get_flag(Flag::Subtract) {
            flags.push_str(" N");
        }
        if self.register_set.get_flag(Flag::HalfCarry) {
            flags.push_str(" H");
        }
        if self.register_set.get_flag(Flag::Carry) {
            flags.push_str(" C");
        }
        write!(f, "registers: {}  |  {}", self.register_set, flags)
    }
}
