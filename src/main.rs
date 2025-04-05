mod cpu;
use cpu::CPU;

mod memory;
use memory::Memory;


fn main() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    memory.write_byte(0x0100, 0x2F);
    memory.write_byte(0x0101, 0x2F);
    memory.write_byte(0x0102, 0x2F); // -3 as an i8 in hex




    for _ in 0..10 {
        // Emulation loop (one step for now)
        let opcode = cpu.fetch_byte(&memory);
        cpu.execute(opcode, &mut memory);
        println!("{}", cpu)
    }



    // println!("CPU State: A = {:#X}", cpu.a);

}
