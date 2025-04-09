mod cpu;
use cpu::CPU;

mod memory;
use memory::Memory;


fn main() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    // memory.write_byte(0x0100, 0x04);

    // memory.write_byte(0x0101, 0xCB);

    // memory.write_byte(0x0102, 0xC0);

    // memory.write_byte(0x0103, 0xCB);
    // memory.write_byte(0x0104, 0x80);

    // memory.write_byte(0x0105, 0x00);
    memory.load_test_rom();





    for _ in 0..10 {
        // Emulation loop (one step for now)
        let opcode = cpu.fetch_byte(&memory);
        println!("0x{:02X}", opcode);
        cpu.execute(opcode, &mut memory);
        println!("{}", cpu);
        cpu.handle_serial_for_test_rom(&mut memory);

        // println!("Register B in binary: {:08b}", cpu.b);
    }



    println!("CPU State: A = {:#X}", cpu.a);

}
