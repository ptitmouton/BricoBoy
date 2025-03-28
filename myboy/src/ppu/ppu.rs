use std::{cell::RefCell, rc::Rc};

use crate::device::mem_map::MemMap;

pub(crate) struct PPU {
    io: Rc<RefCell<MemMap>>,
}

impl PPU {
    pub(crate) fn new(io: Rc<RefCell<MemMap>>) -> PPU {
        PPU { io }
    }
}
