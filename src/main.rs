mod cpu;
mod memory;

use cpu::CPU;
use memory::Memory;


fn main() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    memory.write(0x0100, 0x08);
    memory.write(0x0101, 0x00);
    memory.write(0x0102, 0x01);




    
    // Emulation loop (one step for now)
    let opcode = cpu.fetch(&memory);
    cpu.execute(opcode, &mut memory);



    println!("CPU State: A = {:#X}", cpu.a);
    println!("{}", cpu)
}
