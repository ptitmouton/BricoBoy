use super::{instruction::Instruction, register_set::RegisterSet};
use crate::{cpu::register_set::Flag, device::mem_map::MemMap};
use std::{fmt::Display, num::Wrapping, ops::Add, time::Duration};

pub const CPU_FREQUENCY: u64 = 4_194_304 / 1000; // DBG
pub const M_CYCLE_COUNT: u64 = CPU_FREQUENCY / 4;

pub const M_CYCLE_LENGTH: Duration = Duration::from_nanos(1_000_000_000 / M_CYCLE_COUNT);
pub const T_CYCLE_LENGTH: Duration = Duration::from_nanos(1_000_000_000 / CPU_FREQUENCY);

pub enum InterruptMasterEnableStatus {
    Enabled,
    Enabling,
    Disabled,
}

pub struct CPU {
    pub register_set: RegisterSet,
    pub(crate) current_instruction: Option<Instruction>,
    pub(super) interrupt_master_enable: InterruptMasterEnableStatus,

    cycle_count: Wrapping<u64>,
}

impl CPU {
    pub fn new() -> CPU {
        let register_set = RegisterSet::default();

        CPU {
            register_set,
            interrupt_master_enable: InterruptMasterEnableStatus::Disabled,
            cycle_count: Wrapping(0),
            current_instruction: None,
        }
    }

    pub fn execute(&mut self, mem_map: &mut MemMap) -> u32 {
        self.check_interrupts();

        let next_instruction_address = self.register_set.pc();
        println!(
            "Next instruction address: 0x{:04x}",
            next_instruction_address
        );
        let instruction = Instruction::new(
            mem_map.read_byte(next_instruction_address),
            next_instruction_address,
        );
        let cycles_past = self.run(mem_map, &instruction);
        self.cycle_count = self.cycle_count.add(Wrapping(cycles_past as u64));

        return cycles_past;
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

impl Display for CPU {
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
