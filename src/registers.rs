#![allow(unused)]
use std::fmt;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub g: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    //clock: Clock,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            g: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
            //clock: (),
        }
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(&self.f) as u16
    }

    fn set_af(&mut self, val: u16) {
        self.a = ((val & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((val & 0xFF) as u8);
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, val: u16) {
        self.b = ((val & 0xFF00) >> 8) as u8;
        self.c = (val & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, val: u16) {
        self.d = ((val & 0xFF00) >> 8) as u8;
        self.e = (val & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, val: u16) {
        self.h = ((val & 0xFF00) >> 8) as u8;
        self.l = (val & 0xFF) as u8;
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Registers: \n\
                   \ta: {}\n\
                   \tb: {}\n\
                   \tc: {}\n\
                   \td: {}\n\
                   \te: {}\n\
                   \tf: {}\n\
                   \tg: {}\n\
                   \th: {}\n\
                   \tl: {}\n\
                   \tsp: {}\n\
                   \tpc: {}\n\
                   ",
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.l, self.sp, self.pc
        )
    }
}

pub struct FlagsRegister {
    carry: bool,
    half_carry: bool,
    subtract: bool,
    zero: bool,
}

impl FlagsRegister {
    pub const CARRY_FLAG_BYTE_POSITION: u8 = 4;
    pub const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
    pub const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
    pub const ZERO_FLAG_BYTE_POSITION: u8 = 7;

    pub fn new() -> FlagsRegister {
        FlagsRegister {
            carry: false,
            half_carry: false,
            subtract: false,
            zero: false,
        }
    }
}

impl std::convert::From<&FlagsRegister> for u8 {
    // might be buggy
    fn from(fr: &FlagsRegister) -> u8 {
        let mut output: u8 = 0;

        if fr.carry {
            output |= 0b1 << FlagsRegister::CARRY_FLAG_BYTE_POSITION;
        }

        if fr.half_carry {
            output |= 0b1 << FlagsRegister::HALF_CARRY_FLAG_BYTE_POSITION;
        }

        if fr.subtract {
            output |= 0b1 << FlagsRegister::SUBTRACT_FLAG_BYTE_POSITION;
        }

        if fr.zero {
            output |= 0b1 << FlagsRegister::ZERO_FLAG_BYTE_POSITION;
        }


        output
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(val: u8) -> Self {
        FlagsRegister {
            carry: (val >> FlagsRegister::CARRY_FLAG_BYTE_POSITION) & 0b1 != 0,
            half_carry: (val >> FlagsRegister::HALF_CARRY_FLAG_BYTE_POSITION) & 0b1 != 0,
            subtract: (val >> FlagsRegister::SUBTRACT_FLAG_BYTE_POSITION) & 0b1 != 0,
            zero: (val >> FlagsRegister::ZERO_FLAG_BYTE_POSITION) & 0b1 != 0,
        }
    }
}

impl Default for FlagsRegister {
    fn default() -> Self {
        FlagsRegister::new()
    }
}

impl fmt::Display for FlagsRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\t{}\n\
                   \t\t carry: {}\n\
                   \t\t half_carry: {}\n\
                   \t\t subtract: {}\n\
                   \t\t zero: {}",
            u8::from(self), self.carry as u8, self.half_carry as u8, self.subtract as u8, self.zero as u8,
        )
    }
}
