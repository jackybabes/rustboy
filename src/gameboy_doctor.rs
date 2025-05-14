use crate::cpu::core::CPU;
use crate::memory::Memory;

pub fn gb_doc_print(cpu: &CPU, memory: &Memory) {
    println!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}", 
        cpu.a, cpu.f, cpu.b, cpu.c, cpu.d, cpu.e, cpu.h, cpu.l, 
        cpu.sp, cpu.pc, 
        memory.read_byte(cpu.pc),
        memory.read_byte(cpu.pc + 1),
        memory.read_byte(cpu.pc + 2),
        memory.read_byte(cpu.pc + 3));
}

pub fn gb_doc_set_varibles(cpu: &mut CPU) {
    cpu.a = 0x01;
    cpu.f = 0xB0;
    cpu.b = 0x00;
    cpu.c = 0x13;
    cpu.d = 0x00;
    cpu.e = 0xD8;
    cpu.h = 0x01;
    cpu.l = 0x4D;
    cpu.sp = 0xFFFE;
    cpu.pc = 0x0100;
}

pub fn gb_doc_handle_serial(memory: &mut Memory) {
    let control = memory.read_byte(0xFF02);
    if control == 0x81 {
        let byte = memory.read_byte(0xFF01);
        print!("{}", byte as char); // Output to console
        memory.write_byte(0xFF02, 0x00); // Reset
    }
}

pub fn gb_doc_load_test_rom(memory: &mut Memory) {
    let rom = include_bytes!("/Users/jack/Code/rustboy/roms/gb-test-roms/mem_timing/mem_timing.gb");
    for (i, byte) in rom.iter().enumerate() {
        memory.write_byte(i as u16, *byte);
    }
}