use super::{instruction::Instruction, register_set::RegisterSet};
use crate::{
    Device,
    cpu::register_set::{ByteRegister, Flag, WordRegister},
    device::mem_map::MemMap,
};
use std::{
    fmt::{Debug, Display},
    num::Wrapping,
    ops::AddAssign,
    time::Duration,
};

pub const CPU_FREQUENCY: u64 = 4_194_304; // DBG

pub const CYCLE_LENGTH: Duration = Duration::from_nanos(1_000_000_000 / CPU_FREQUENCY);

#[derive(Debug, Clone, Copy)]
pub enum InterruptMasterEnableStatus {
    Enabled,
    Enabling,
    Disabled,
}

pub struct CPU {
    pub register_set: RegisterSet,
    pub(crate) current_instruction: Option<Instruction>,
    pub(super) interrupt_master_enable: InterruptMasterEnableStatus,

    cycle_counter: Wrapping<u8>,
    occupied_cycles: u32,
}

#[derive(Clone, Copy)]
pub struct CPUState {
    pub register_set: RegisterSet,
    pub interrupt_master_enable: InterruptMasterEnableStatus,
    pub current_instruction: Instruction,
    pub current_instruction_bytes: [u8; 4],
}

impl Debug for CPUState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pc = self.register_set.pc();
        let regset = &self.register_set;

        // let format1: &str = "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})";
        // let format2: &str = "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:00:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}";

        write!(
            f,
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:00:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            regset.get_b(ByteRegister::A),
            regset.get_b(ByteRegister::F),
            regset.get_b(ByteRegister::B),
            regset.get_b(ByteRegister::C),
            regset.get_b(ByteRegister::D),
            regset.get_b(ByteRegister::E),
            regset.get_b(ByteRegister::H),
            regset.get_b(ByteRegister::L),
            regset.get_w(WordRegister::SP),
            pc,
            self.current_instruction_bytes[0],
            self.current_instruction_bytes[1],
            self.current_instruction_bytes[2],
            self.current_instruction_bytes[3],
        )
    }
}

impl From<&Device> for CPUState {
    fn from(device: &Device) -> Self {
        let cpu = &device.cpu;
        let register_set = cpu.register_set.clone();
        let interrupt_master_enable = cpu.interrupt_master_enable;

        let pc = (*cpu.register_set.pc()).clone();
        let current_instruction = cpu.current_instruction.unwrap_or_default();
        let current_bytes = device.mem_map.read_word(pc).to_le_bytes();
        let next_bytes = device.mem_map.read_word(pc + 2).to_le_bytes();

        let current_instruction_bytes = [
            current_bytes[0],
            current_bytes[1],
            next_bytes[0],
            next_bytes[1],
        ];

        CPUState {
            register_set,
            interrupt_master_enable,
            current_instruction,
            current_instruction_bytes,
        }
    }
}

impl CPU {
    pub fn new() -> CPU {
        let register_set = RegisterSet::default();

        CPU {
            register_set,
            interrupt_master_enable: InterruptMasterEnableStatus::Disabled,
            cycle_counter: Wrapping(0),
            occupied_cycles: 0,
            current_instruction: None,
        }
    }

    pub fn cycle(&mut self, mem_map: &mut MemMap) {
        self.cycle_counter.add_assign(1);
        if self.cycle_counter.0 == 0 {
            // every 256 cycles
            mem_map.io_registers.inc_timer_div();
        }
        if (self.cycle_counter.0 & 0b11) == 0x0 {
            // every 4 cycles
            self.m_cycle(mem_map);
        }
    }

    pub fn m_cycle(&mut self, mem_map: &mut MemMap) {
        // a CPU m-cycle (= 4 cycles)
        if self.occupied_cycles != 0 {
            self.occupied_cycles -= 1;
            return;
        }
        self.check_interrupts();

        let next_instruction_address = *self.register_set.pc();
        let instruction = Instruction::create(next_instruction_address, mem_map).unwrap();
        self.occupied_cycles = self.run(mem_map, &instruction) - 1;
    }

    pub fn is_busy(&self) -> bool {
        self.occupied_cycles != 0
    }

    pub(super) fn push_to_stack(&mut self, mem_map: &mut MemMap, value: u16) {
        let sp = *self.register_set.sp();
        mem_map.write_byte(sp - 1, (value >> 8) as u8);
        mem_map.write_byte(sp - 2, (value & 0xff) as u8);
        self.register_set.set_sp(sp - 2);
    }

    pub(super) fn pop_from_stack(&mut self, mem_map: &mut MemMap) -> u16 {
        let sp = *self.register_set.sp();
        let value = mem_map.read_word(sp);
        self.register_set.set_sp(sp + 2);
        value
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
