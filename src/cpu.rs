use std::fmt;
use crate::registers::Registers;

#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
    registers: Registers,
    uptime: u32,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            uptime: 0
        }
    }

    pub fn exec(&mut self, opcode: u8) {
        let mut regs = &self.registers;
        match opcode {
            0x00 => { }
            0x01 => {  }
            _ => {}
        }
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cpu state: \n{}", self.registers)
    }
}

mod instructions {
    pub struct Instruction {
        //opcode:
    }
}
