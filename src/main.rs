mod cpu;
use cpu::CPU;

mod memory;
use memory::Memory;


fn main() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    memory.write_byte(0x0100, 0xCB);

    memory.write_byte(0x0101, 0x00);





    for _ in 0..10 {
        // Emulation loop (one step for now)
        let opcode = cpu.fetch_byte(&memory);
        println!("0x{:02X}", opcode);
        cpu.execute(opcode, &mut memory);
        println!("{}", cpu)
    }



    println!("CPU State: A = {:#X}", cpu.a);

}
