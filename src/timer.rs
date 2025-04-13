use crate::memory::Memory;
use crate::data::HardwareRegister;

pub struct Timer {
    div_counter: u16,
    tima_counter: u16,
}

impl Timer {
    pub fn new(memory: &mut Memory) -> Self {
        // Initialize memory-mapped timer registers to default boot values
        memory.write_hardware_register(HardwareRegister::DIV, 0x00);
        memory.write_hardware_register(HardwareRegister::TIMA, 0x00);
        memory.write_hardware_register(HardwareRegister::TMA, 0x00);
        memory.write_hardware_register(HardwareRegister::TAC, 0x00);

        Timer {
            div_counter: 0,
            tima_counter: 0,
        }
    }
    
    pub fn step(&mut self, cycles: u16, memory: &mut Memory) {
        // DIV (0xFF04) — increments every 256 cycles
        self.div_counter += cycles;
        while self.div_counter >= 256 {
            self.div_counter -= 256;
            let current_div = memory.read_hardware_register(HardwareRegister::DIV);
            memory.write_hardware_register(HardwareRegister::DIV, current_div.wrapping_add(1));
        }

        // Read TAC (0xFF07)
        let tac = memory.read_hardware_register(HardwareRegister::TAC);
        let timer_enabled = tac & 0b100 != 0;

        if timer_enabled {
            let threshold = match tac & 0b11 {
                0b00 => 1024,
                0b01 => 16,
                0b10 => 64,
                0b11 => 256,
                _ => unreachable!(),
            };

            self.tima_counter += cycles;
            while self.tima_counter >= threshold {
                self.tima_counter -= threshold;

                let mut tima = memory.read_hardware_register(HardwareRegister::TIMA);
                tima = tima.wrapping_add(1);

                if tima == 0 {
                    // Overflow occurred — reload TIMA from TMA and request interrupt
                    let tma = memory.read_hardware_register(HardwareRegister::TMA);
                    memory.write_hardware_register(HardwareRegister::TIMA, tma);

                    let mut if_reg = memory.read_hardware_register(HardwareRegister::IF);
                    if_reg |= 0b0000_0100;
                    memory.write_hardware_register(HardwareRegister::IF, if_reg);
                } else {
                    memory.write_hardware_register(HardwareRegister::TIMA, tima);
                }
            }
        }
    }
}
