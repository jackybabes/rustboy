use crate::cpu::CPU;
use crate::memory::Memory;

pub enum Interrupt {
    VBlank = 0x40,
    LCDStat = 0x48,
    Timer = 0x50,
    Serial = 0x58,
    Joypad = 0x60,
}

pub fn handle_interrupt(cpu: &mut CPU, memory: &mut Memory, interrupt: u8) {
    let addr = match interrupt {
        0 => Interrupt::VBlank as u16, // V-Blank
        1 => Interrupt::LCDStat as u16, // LCD STAT
        2 => Interrupt::Timer as u16, // Timer
        3 => Interrupt::Serial as u16, // Serial
        4 => Interrupt::Joypad as u16, // Joypad
        _ => return,
    };

    // Push current PC to stack
    cpu.push_u16(memory, cpu.pc);
    cpu.pc = addr;

    cpu.cycles += 20;
}