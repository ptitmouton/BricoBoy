use std::{fmt::Display, u8};

#[derive(Clone, Copy, Debug)]
pub(crate) enum ByteRegister {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum WordRegister {
    AF,
    BC,
    DE,
    HL,
    HLi,
    HLd,

    PC,
    SP,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Flag {
    Zero,
    Subtract,
    HalfCarry,
    Carry,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RegisterSet {
    a: u8,
    f: u8,

    b: u8,
    c: u8,

    d: u8,
    e: u8,

    h: u8,
    l: u8,

    pc: u16,
    sp: u16,
}

impl Default for RegisterSet {
    fn default() -> Self {
        RegisterSet {
            a: 0x01,
            f: 0xb0,

            b: 0x00,
            c: 0x13,

            d: 0x00,
            e: 0xd8,

            h: 0x01,
            l: 0x4d,

            pc: 0x0100,
            sp: 0xfffe,
        }
    }
}

impl RegisterSet {
    pub(crate) fn get_b(&self, register: ByteRegister) -> &u8 {
        match register {
            ByteRegister::A => &self.a,
            ByteRegister::F => &self.f,

            ByteRegister::B => &self.b,
            ByteRegister::C => &self.c,

            ByteRegister::D => &self.d,
            ByteRegister::E => &self.e,

            ByteRegister::H => &self.h,
            ByteRegister::L => &self.l,
        }
    }
    pub(crate) fn get_w(&self, register: WordRegister) -> u16 {
        match register {
            WordRegister::AF => (self.a as u16) << 8 | (self.f as u16),
            WordRegister::BC => (self.b as u16) << 8 | (self.c as u16),
            WordRegister::DE => (self.d as u16) << 8 | (self.e as u16),
            WordRegister::HL | WordRegister::HLi | WordRegister::HLd => {
                (self.h as u16) << 8 | (self.l as u16)
            }

            WordRegister::PC => self.pc,
            WordRegister::SP => self.sp,
        }
    }
    pub(crate) fn set_b(&mut self, register: ByteRegister, value: u8) -> &u8 {
        match register {
            ByteRegister::A => {
                self.a = value;
                &self.a
            }
            ByteRegister::F => {
                self.f = value;
                &self.f
            }

            ByteRegister::B => {
                self.b = value;
                &self.b
            }
            ByteRegister::C => {
                self.c = value;
                &self.c
            }

            ByteRegister::D => {
                self.d = value;
                &self.d
            }
            ByteRegister::E => {
                self.e = value;
                &self.e
            }

            ByteRegister::H => {
                self.h = value;
                &self.h
            }
            ByteRegister::L => {
                self.l = value;
                &self.l
            }
        }
    }
    pub(crate) fn set_w(&mut self, register: WordRegister, value: u16) {
        match register {
            WordRegister::AF => {
                let byte_values = value.to_le_bytes();
                self.a = byte_values[1];
                self.f = byte_values[0];
            }
            WordRegister::BC => {
                let byte_values = value.to_le_bytes();
                self.b = byte_values[1];
                self.c = byte_values[0];
            }
            WordRegister::DE => {
                let byte_values = value.to_le_bytes();
                self.d = byte_values[1];
                self.e = byte_values[0];
            }
            WordRegister::HL | WordRegister::HLi | WordRegister::HLd => {
                let byte_values = value.to_le_bytes();
                self.h = byte_values[1];
                self.l = byte_values[0];
            }

            WordRegister::PC => {
                self.pc = value;
            }
            WordRegister::SP => {
                self.sp = value;
            }
        }
    }

    pub(crate) fn get_flag(&self, flag: Flag) -> bool {
        let mask = match flag {
            Flag::Zero => 0b1000_0000,
            Flag::Subtract => 0b0100_0000,
            Flag::HalfCarry => 0b0010_0000,
            Flag::Carry => 0b0001_0000,
        };

        self.f & mask != 0
    }

    pub(crate) fn set_flag(&mut self, flag: Flag, value: bool) {
        let mask = match flag {
            Flag::Zero => 0b1000_0000,
            Flag::Subtract => 0b0100_0000,
            Flag::HalfCarry => 0b0010_0000,
            Flag::Carry => 0b0001_0000,
        };

        if value {
            self.f |= mask;
        } else {
            self.f &= !mask;
        }
    }

    pub(super) fn sp(&self) -> &u16 {
        &self.sp
    }

    pub(super) fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    pub fn pc(&self) -> &u16 {
        &self.pc
    }
}

impl Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a: 0x{:02x}, f: 0x{:02x}, b: 0x{:02x}, c: 0x{:02x}, d: 0x{:02x}, e: 0x{:02x}, h: 0x{:02x}, l: 0x{:02x}, pc: 0x{:04x}, sp: 0x{:04x}\n",
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.pc, self.sp
        )
    }
}

impl Display for ByteRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteRegister::A => write!(f, "A"),
            ByteRegister::F => write!(f, "F"),
            ByteRegister::B => write!(f, "B"),
            ByteRegister::C => write!(f, "C"),
            ByteRegister::D => write!(f, "D"),
            ByteRegister::E => write!(f, "E"),
            ByteRegister::H => write!(f, "H"),
            ByteRegister::L => write!(f, "L"),
        }
    }
}

impl Display for WordRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WordRegister::AF => write!(f, "AF"),
            WordRegister::BC => write!(f, "BC"),
            WordRegister::DE => write!(f, "DE"),
            WordRegister::HL => write!(f, "HL"),
            WordRegister::HLi => write!(f, "HLi"),
            WordRegister::HLd => write!(f, "HLd"),
            WordRegister::PC => write!(f, "PC"),
            WordRegister::SP => write!(f, "SP"),
        }
    }
}
