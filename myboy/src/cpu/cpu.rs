use super::{instruction::Instruction, register_set::RegisterSet};
use crate::{
    Logger,
    cpu::register_set::{Flag, WordRegister},
    device::mem_map::MemMap,
    io::if_register::{InterruptType, get_handler_address},
    logging::log::Log,
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
    pub current_instruction: Option<Instruction>,
    pub interrupt_master_enable: InterruptMasterEnableStatus,
    pub halted: bool,
    pub logger: Option<Box<dyn Logger>>,

    stopped: bool,
    cycle_counter: Wrapping<u8>,
    occupied_cycles: u32,
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
            stopped: false,
            halted: false,
            logger: Option::None,
        }
    }

    pub fn cycle(&mut self, mem_map: &mut MemMap, logger: &mut dyn Logger) {
        self.cycle_counter.add_assign(1);
        if !self.stopped {
            if (self.cycle_counter.0 & 0b11) == 0x0 {
                // every 4 cycles
                self.m_cycle(mem_map, logger);
            }
        }
        mem_map.io_registers.update_timers();
    }

    pub fn m_cycle(&mut self, mem_map: &mut MemMap, logger: &mut dyn Logger) {
        // a CPU m-cycle (= 4 cycles)
        if self.occupied_cycles != 0 {
            self.occupied_cycles -= 1;
            return;
        }
        let halted = self.halted;
        if !halted {
            logger.info(Log::CPUState(super::CPUState::new(self, mem_map)));
        }
        self.check_interrupts(mem_map);
        if halted && !self.halted {
            logger.info(Log::CPUState(super::CPUState::new(self, mem_map)));
        }

        let next_instruction_address = *self.register_set.pc();
        let instruction = Instruction::create(next_instruction_address, mem_map).unwrap();
        if !self.halted {
            self.occupied_cycles = self.run(mem_map, &instruction) - 1;
        }
    }

    #[inline]
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn is_busy(&self) -> bool {
        self.occupied_cycles != 0
    }

    pub(super) fn push_to_stack(&mut self, mem_map: &mut MemMap, value: u16) {
        let sp = *self.register_set.sp();
        let byte_values = value.to_le_bytes();
        mem_map.write_byte(sp - 1, byte_values[1]);
        mem_map.write_byte(sp - 2, byte_values[0]);
        self.register_set.set_sp(sp - 2);
    }

    pub(super) fn pop_from_stack(&mut self, mem_map: &mut MemMap) -> u16 {
        let sp = *self.register_set.sp();
        let value = mem_map.read_word(sp);
        self.register_set.set_sp(sp + 2);
        value
    }

    fn check_interrupts(&mut self, mem_map: &mut MemMap) {
        let if_reg = &mut mem_map.io_registers.if_register;
        let ie_ref = &mem_map.io_registers.ie_register;

        if if_reg.0 & ie_ref.0 & 0b0001_1111 != 0 {
            self.halted = false;
        }

        match self.interrupt_master_enable {
            InterruptMasterEnableStatus::Enabled => {
                // Check for interrupts
                if if_reg.is_requested(InterruptType::Timer) {
                    if_reg.clear_request(InterruptType::Timer);
                    if ie_ref.is_timer_handler_enabled() {
                        self.trigger_interrupt_handler(mem_map, InterruptType::Timer);
                    }
                }

                return;
            }
            InterruptMasterEnableStatus::Enabling => {
                self.interrupt_master_enable = InterruptMasterEnableStatus::Enabled
            }
            InterruptMasterEnableStatus::Disabled => {}
        }
    }

    pub fn trigger_interrupt_handler(
        &mut self,
        mem_map: &mut MemMap,
        interrupt_type: InterruptType,
    ) {
        self.interrupt_master_enable = InterruptMasterEnableStatus::Disabled;
        self.push_to_stack(mem_map, *self.register_set.pc());
        self.register_set
            .set_w(WordRegister::PC, get_handler_address(interrupt_type));

        self.occupied_cycles += 5;
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
