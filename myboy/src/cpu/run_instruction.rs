use crate::{cpu::cpu::InterruptMasterEnableStatus, io::if_register::IFRegister, logger};

use super::{
    addressing_mode::AddressingMode,
    condition::Condition,
    cpu::CPU,
    instruction::{Instruction, InstructionType},
    register_set::{Flag, WordRegister},
};

impl CPU {
    pub(super) fn get_source_byte(&self, instruction: &Instruction) -> u8 {
        let io = self.io.borrow();
        match instruction.source {
            Some(AddressingMode::ImmediateByte) => io.read_byte(instruction.address + 1),
            Some(AddressingMode::ByteRegister(register)) => *self.register_set.get_b(register),
            Some(AddressingMode::Value(value)) => value,
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                io.read_byte(address)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = io.read_word(instruction.address + 1);
                io.read_byte(address)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = 0xff00 + (io.read_byte(instruction.address + 1) as u16);
                println!(
                    "offset: 0x{:04x}",
                    (io.read_byte(instruction.address + 1) as u16)
                );
                println!("address: 0x{:04x}", address);
                let content = io.read_byte(address);
                println!("content: 0x{:04x}", address);
                content
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = (0xff00 as u16) & (*self.register_set.get_b(register)) as u16;
                io.read_byte(address)
            }
            _ => panic!("No source provided for instruction"),
        }
    }

    pub(super) fn get_source_word(&self, instruction: &Instruction) -> u16 {
        let io = self.io.borrow_mut();
        match instruction.source {
            Some(AddressingMode::ImmediateWord) => io.read_word(instruction.address + 1),
            Some(AddressingMode::WordRegister(register)) => self.register_set.get_w(register),
            Some(AddressingMode::Value(value)) => value.into(),
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                io.read_word(address)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = io.read_word(instruction.address + 1);
                io.read_word(address)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = 0xff00 & io.read_byte(instruction.address + 1) as u16;
                io.read_word(address)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & *self.register_set.get_b(register) as u16;
                io.read_word(address)
            }
            _ => panic!("No source provided for instruction"),
        }
    }

    pub(super) fn get_target_byte(&self, instruction: &Instruction) -> u8 {
        let io = self.io.borrow();
        match instruction.target {
            Some(AddressingMode::ImmediateByte) => io.read_byte(instruction.address + 1),
            Some(AddressingMode::ByteRegister(register)) => *self.register_set.get_b(register),
            Some(AddressingMode::Value(value)) => value,
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                io.read_byte(address)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = io.read_word(instruction.address + 1);
                io.read_byte(address)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = (0xff00 as u16) & (io.read_byte(instruction.address + 1) as u16);
                io.read_byte(address)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & (*self.register_set.get_b(register) as u16);
                io.read_byte(address)
            }
            _ => panic!("No target provided for instruction"),
        }
    }

    pub(super) fn get_target_word(&self, instruction: &Instruction) -> u16 {
        let io = self.io.borrow();
        match instruction.target {
            Some(AddressingMode::ImmediateWord) => io.read_word(instruction.address + 1),
            Some(AddressingMode::WordRegister(register)) => self.register_set.get_w(register),
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                io.read_word(address)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = io.read_word(instruction.address + 1);
                io.read_word(address)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = 0xff00 & io.read_word(instruction.address + 1);
                io.read_word(address)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & (*self.register_set.get_b(register) as u16);
                io.read_word(address)
            }
            _ => {
                logger::error!("No target provided for instruction: {}", instruction);
                panic!("No target provided for instruction!");
            }
        }
    }

    pub(super) fn write_target_byte(&mut self, instruction: &Instruction, value: u8) {
        let mut io = self.io.borrow_mut();
        match instruction.target {
            Some(AddressingMode::ImmediateByte) => io.write_byte(instruction.address + 1, value),
            Some(AddressingMode::ByteRegister(register)) => {
                self.register_set.set_b(register, value);
            }
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                io.write_byte(address, value)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = io.read_word(instruction.address + 1);
                io.write_byte(address, value)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = (0xff00) & (io.read_byte(instruction.address + 1)) as u16;
                io.write_byte(address, value)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & (*self.register_set.get_b(register)) as u16;
                io.write_byte(address, value)
            }
            _ => panic!("No source provided for instruction"),
        }
    }

    pub(super) fn write_target_word(&mut self, instruction: &Instruction, value: u16) {
        let mut io = self.io.borrow_mut();
        match instruction.target {
            Some(AddressingMode::ImmediateWord) => io.write_word(instruction.address + 1, value),
            Some(AddressingMode::WordRegister(register)) => {
                self.register_set.set_w(register, value)
            }
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                io.write_word(address, value)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = io.read_word(instruction.address + 1);
                io.write_word(address, value)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = (0xff00) & (io.read_byte(instruction.address + 1)) as u16;
                io.write_word(address, value)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & (*self.register_set.get_b(register) as u16);
                io.write_word(address, value)
            }
            // We can also use 8-bit targets in 16-bit operations, ops like LDH do require
            // it, so we need to handle it here.
            // Normally the 16bit source is an address, so writing to it is absolutely possible.
            Some(AddressingMode::ByteRegister(register)) => {
                self.register_set.set_b(register, value as u8);
            }
            _ => panic!(
                "No (u16) target provided for instruction: {}",
                instruction.target.unwrap()
            ),
        }
    }

    pub(super) fn run(&mut self, instruction: &Instruction) -> u32 {
        self.register_set
            .set_w(WordRegister::PC, instruction.address);

        // logger::info!(
        //     "executing: {}",
        //     self.print_detailed_instruction(instruction)
        // );

        match instruction.instruction_type {
            InstructionType::Nop => {
                self.register_set.set_flag(Flag::Zero, false);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set
                    .set_w(WordRegister::PC, instruction.address + 1);
                return 1;
            }
            InstructionType::LoadByte => {
                let source = self.get_source_byte(instruction);
                self.write_target_byte(instruction, source);
                self.register_set.set_flag(Flag::Zero, false);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::LoadWord => {
                let source = self.get_source_word(instruction);
                self.write_target_word(instruction, source);
                match instruction.target {
                    Some(AddressingMode::WordRegister(WordRegister::HLi)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) + 1,
                        );
                    }
                    Some(AddressingMode::WordRegister(WordRegister::HLd)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) - 1,
                        );
                    }
                    _ => {}
                }
                self.register_set.set_flag(Flag::Zero, false);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::LoadHigh => {
                let source = self.get_source_byte(instruction) as u16;
                self.write_target_word(instruction, source);
                self.register_set.set_flag(Flag::Zero, source == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::Or => {
                let source = self.get_source_byte(instruction);
                let target = self.get_target_byte(instruction);
                let value = source | target;
                self.write_target_byte(instruction, value);
                self.register_set.set_flag(Flag::Zero, value == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::And => {
                let source = self.get_source_byte(instruction);
                let target = self.get_target_byte(instruction);
                self.write_target_byte(instruction, source & target);
                self.register_set
                    .set_flag(Flag::Zero, (source & target) == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::Xor => {
                let source = self.get_source_byte(instruction);
                let target = self.get_target_byte(instruction);
                self.write_target_byte(instruction, source ^ target);
                self.register_set.set_flag(Flag::Zero, source == target);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::AddByte => {
                let source = self.get_source_byte(instruction);
                let target = self.get_target_byte(instruction);
                self.write_target_byte(instruction, source + target);
                self.register_set
                    .set_flag(Flag::Zero, (source + target) == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set
                    .set_flag(Flag::HalfCarry, (source & 0xf) + (target & 0xf) > 0xf);
                self.register_set
                    .set_flag(Flag::Carry, (source as u16) + (target as u16) > 0xff);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::AddWord => {
                let source = self.get_source_word(instruction);
                let target = self.get_target_word(instruction);
                let result = source.wrapping_add(target);
                self.write_target_word(instruction, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set
                    .set_flag(Flag::HalfCarry, (source & 0xf) + (target & 0xf) > 0xf);
                self.register_set
                    .set_flag(Flag::Carry, (source as u32) + (target as u32) > 0xff);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::Sub => {
                let source = self.get_source_byte(instruction);
                let target = self.get_target_byte(instruction);
                self.write_target_byte(instruction, source - target);
                self.register_set.set_flag(Flag::Zero, source == target);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::HalfCarry, (source & 0xf) < (target & 0xf));
                self.register_set.set_flag(Flag::Carry, source < target);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::IncByte => {
                let target = self.get_target_byte(instruction);
                self.write_target_byte(instruction, target.wrapping_add(1));
                self.register_set.set_flag(Flag::Zero, target == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set
                    .set_flag(Flag::HalfCarry, (target & 0xf) == 0xf);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::DecByte => {
                let target = self.get_target_byte(instruction);
                let new_target = target.wrapping_sub(1);
                self.write_target_byte(instruction, new_target);
                self.register_set.set_flag(Flag::Zero, new_target == 0);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::HalfCarry, (new_target & 0xf) == 0);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::Jump => {
                let target = self.get_target_word(instruction);
                self.register_set.reset_flags();
                self.register_set.set_w(WordRegister::PC, target);
                return 4;
            }
            InstructionType::JumpRelative => {
                let condition_met = self.condition_met(instruction);
                self.register_set.reset_flags();
                if condition_met {
                    let target = self.get_target_byte(instruction);
                    let current =
                        self.register_set.get_w(WordRegister::PC) + instruction.size() as u16;
                    // println!(
                    //     "target: u16: {}({:02x}) i8: {}({:02x}) u16:{}({:02x}) #{:08x}=>#{:08x}",
                    //     target as u16,
                    //     target as u16,
                    //     target as i8,
                    //     target as i8,
                    //     target as i8 as u16,
                    //     target as i8 as u16,
                    //     current,
                    //     current.wrapping_add(target as i8 as u16)
                    // );
                    let result = current.wrapping_add(target as i8 as u16);
                    self.register_set.set_w(WordRegister::PC, result);
                    return 4;
                }
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + instruction.size() as u16,
                );
                return 3;
            }
            InstructionType::Push => {
                let source = self.get_source_word(instruction);
                self.register_set.set_w(
                    WordRegister::SP,
                    self.register_set.get_w(WordRegister::SP) - 2,
                );
                self.io
                    .borrow_mut()
                    .write_word(self.register_set.get_w(WordRegister::SP), source);
                self.register_set.reset_flags();
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 4;
            }
            InstructionType::Pop => {
                let value = self
                    .io
                    .borrow()
                    .read_word(self.register_set.get_w(WordRegister::SP));
                self.register_set.set_w(
                    WordRegister::SP,
                    self.register_set.get_w(WordRegister::SP) + 2,
                );
                self.write_target_word(instruction, value);
                self.register_set.reset_flags();
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 3;
            }
            InstructionType::DisableInterrupts => {
                self.interrupt_master_enable = InterruptMasterEnableStatus::Disabled;
                self.register_set.reset_flags();
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::EnableInterrupts => {
                self.interrupt_master_enable = InterruptMasterEnableStatus::Enabling;
                self.register_set.reset_flags();
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::Cp => {
                let source = self.get_source_byte(instruction);
                let target = self.get_target_byte(instruction);
                let result = target.wrapping_sub(source);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::HalfCarry, (target & 0xf) < (source & 0xf));
                self.register_set.set_flag(Flag::Carry, target < source);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::RollRight => {
                let target = self.get_target_byte(instruction);
                let result = (target >> 1) | (target << 7);
                self.write_target_byte(instruction, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, target & 0x1 == 1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::RollRightThroughCarry => {
                let target = self.get_target_byte(instruction);
                let carry = self.register_set.get_flag(Flag::Carry) as u8;
                let result = (target >> 1) | (carry << 7);
                self.write_target_byte(instruction, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, target & 0x1 == 1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::RollLeft => {
                let target = self.get_target_byte(instruction);
                let result = (target << 1) | (target >> 7);
                self.write_target_byte(instruction, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, target & 0x1 == 1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::Stop => {
                self.interrupt_master_enable = InterruptMasterEnableStatus::Disabled;
                self.io
                    .borrow_mut()
                    .io_registers
                    .set_if_register(IFRegister(0x00));
                self.register_set.reset_flags();
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            instruction_type => {
                todo!(
                    "The instruction type {} is not implemented",
                    instruction_type
                );
            }
        };
    }

    fn pretty_print_addressing(
        &self,
        instruction: &Instruction,
        addressing: &AddressingMode,
    ) -> String {
        let io = self.io.borrow();
        return match addressing {
            AddressingMode::ImmediateByte => format!(
                "imm8([0x{:04x}+1]: 0x{:02x})",
                instruction.address,
                io.read_byte(instruction.address + 1)
            ),
            AddressingMode::ImmediateWord => format!(
                "imm8([0x{:04x}+1]: 0x{:04x})",
                instruction.address,
                io.read_word(instruction.address + 1)
            ),
            AddressingMode::ImmediatePointer => {
                let pointer = io.read_word(instruction.address + 1);
                let value = io.read_byte(pointer);
                format!(
                    "immPtr([[0x{:04x}+1] -> [0x{:04x}]]: 0x{:02x})",
                    instruction.address, pointer, value
                )
            }
            AddressingMode::ImmediatePointerHigh => {
                let pointer_high = io.read_byte(instruction.address + 1);
                let pointer = (io.read_byte(instruction.address + 1) as u16) & 0xff00;
                let value = io.read_byte(pointer);
                format!(
                    "immPtrHigh([[0x{:02x} & 0xff00] ({:02x} & 0xff00) -> [0x{:04x}]]: 0x{:04x})",
                    instruction.address + 1,
                    pointer_high,
                    pointer,
                    value
                )
            }
            AddressingMode::ByteRegister(register) => {
                format!("{}: {:02x}", register, self.register_set.get_b(*register))
            }
            AddressingMode::WordRegister(register) => {
                format!("{}: {:04x}", register, self.register_set.get_w(*register))
            }
            AddressingMode::RegisterPointer(register) => {
                let pointer = self.register_set.get_w(*register);
                let value = io.read_byte(pointer);
                format!("[{}] -> {:04x}: {:02x}", register, pointer, value)
            }
            AddressingMode::RegisterPointerHigh(register) => {
                let reg_value = *self.register_set.get_b(*register) as u16;
                let pointer = (0xff00 as u16) & reg_value;
                let value = io.read_byte(pointer);
                format!(
                    "[0xff00 & {}] -> [0xff00 & {:02x}] -> {:04x}: {:02x}",
                    register, reg_value, pointer, value
                )
            }
            AddressingMode::Value(value) => format!("val: 0x{:02x}", value),
        };
    }

    fn condition_met(&mut self, instruction: &Instruction) -> bool {
        match instruction.condition {
            Some(condition) => match condition {
                Condition::NotZero => !self.register_set.get_flag(Flag::Zero),
                Condition::Zero => self.register_set.get_flag(Flag::Zero),
                Condition::NoCarry => !self.register_set.get_flag(Flag::Carry),
                Condition::Carry => self.register_set.get_flag(Flag::Carry),
            },
            _ => true,
        }
    }

    fn print_detailed_instruction(&self, instruction: &Instruction) -> String {
        let instruction_type = format!("{:?}", instruction.instruction_type);

        let source = match instruction.source {
            Some(source) => self.pretty_print_addressing(instruction, &source),
            None => "".to_string(),
        };
        let target = match instruction.target {
            Some(target) => self.pretty_print_addressing(instruction, &target),
            None => "".to_string(),
        };

        let mut result = String::new();
        for (_i, s) in [target, source].iter().enumerate() {
            if s.len() > 0 {
                if result.len() > 0 {
                    result.push_str(" < ");
                }
                result.push_str(s);
            }
        }
        if result.len() > 0 {
            format!("{}: {}", instruction_type, result)
        } else {
            format!("{}", instruction_type)
        }
    }
}
