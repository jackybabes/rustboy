mod cpu;
mod memory;

use cpu::CPU;
use memory::Memory;


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
