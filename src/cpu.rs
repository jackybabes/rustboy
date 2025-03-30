use crate::memory::Memory;
// Define CPU registers
pub struct CPU {
    pub a: u8, pub f: u8, // Accumulator & Flags
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
}

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CPU {{ A: {}, F: {}, B: {}, C: {}, D: {}, E: {}, H: {}, L: {}, SP: {}, PC: {} }}", 
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc)
    }
}


impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub fn fetch(&mut self, memory: &Memory) -> u8 {
        let opcode = memory.read(self.pc);
        self.pc += 1;
        opcode
    }

    pub fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => {}, // NOP - Do nothing
            0x3C => self.a = self.a.wrapping_add(1), // INC A
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
    }
}
