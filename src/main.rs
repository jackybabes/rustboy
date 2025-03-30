use std::collections::HashMap;

// Define CPU registers
struct CPU {
    a: u8, f: u8, // Accumulator & Flags
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8,
    sp: u16, // Stack Pointer
    pc: u16, // Program Counter
}

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CPU {{ A: {}, F: {}, B: {}, C: {}, D: {}, E: {}, H: {}, L: {}, SP: {}, PC: {} }}", 
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc)
    }
}


impl CPU {
    fn new() -> Self {
        CPU {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    fn fetch(&mut self, memory: &Memory) -> u8 {
        let opcode = memory.read(self.pc);
        self.pc += 1;
        opcode
    }

    fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => {}, // NOP - Do nothing
            0x3C => self.a = self.a.wrapping_add(1), // INC A
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
    }
}

// Define Memory Management Unit
struct Memory {
    data: [u8; 0x10000],
}

impl Memory {
    fn new() -> Self {
        Memory { data: [0; 0x10000] }
    }

    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

fn main() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    // Load a test instruction into memory
    memory.write(0x0100, 0x3C); // INC A
    
    // Emulation loop (one step for now)
    let opcode = cpu.fetch(&memory);
    cpu.execute(opcode);
    println!("CPU State: A = {:#X}", cpu.a);
    println!("{}", cpu)
}
