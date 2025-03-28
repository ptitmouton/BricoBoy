use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Condition {
    NotZero,
    Zero,
    NoCarry,
    Carry,
}

impl Condition {
    pub(crate) fn get_condition(bits: u8) -> Option<Condition> {
        match bits & 0b0000_0011 {
            0b00 => Some(Condition::NotZero),
            0b01 => Some(Condition::Zero),
            0b10 => Some(Condition::NoCarry),
            0b11 => Some(Condition::Carry),
            _ => None,
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::NotZero => write!(f, "NZ"),
            Condition::Zero => write!(f, "Z"),
            Condition::NoCarry => write!(f, "NC"),
            Condition::Carry => write!(f, "C"),
        }
    }
}
