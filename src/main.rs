mod cpu;
mod memory;
mod data;
mod interrupts;
mod timer;

use cpu::CPU;
use memory::Memory;
use data::HardwareRegister;
use interrupts::handle_interrupt;
use timer::Timer;

pub struct GameBoy {
    pub cpu: CPU,
    pub memory: Memory,
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy {
            cpu: CPU::new(),
            memory: Memory::new(),
        }
    }

    pub fn step(&mut self) {
        // handle interupts
        if self.cpu.interrupts.ime {
            let ie = self.memory.read_hardware_register(HardwareRegister::IE);
            let mut if_ = self.memory.read_hardware_register(HardwareRegister::IF);
            let triggered = ie & if_;
            
            if triggered != 0 {
                for i in 0..5 {
                    if triggered & (1 << i) != 0 {
                        if_ &= !(1 << i);
                        self.memory.write_hardware_register(HardwareRegister::IF, if_);

                        handle_interrupt(&mut self.cpu, &mut self.memory, i);

                        self.cpu.interrupts.ime = false;
                        return;
                    }
                }
            }
        } 
        // Deal with Halt
        if self.cpu.is_halted {
            // ⚠️ Special case: HALT bug
            let ie = self.memory.read_hardware_register(HardwareRegister::IE);
            let iflag = self.memory.read_hardware_register(HardwareRegister::IF);
            let pending = ie & iflag;
    
            if pending != 0 {
                // HALT bug: IME is off, but an interrupt is pending
                self.cpu.is_halted = false;
            } else {
                // stay halted
                // self.cpu.print_gameboy_doc_output(&mut self.memory);

                self.cpu.cycles = 4;
                return;
            }
        }

        // get next op code this increments the pc by 1
        let opcode = self.cpu.fetch_byte(&self.memory);
        // execute op code  
        self.cpu.execute(opcode, &mut self.memory);


        // delayed enable interuopt
        if self.cpu.interrupts.enable_ime_next {
            self.cpu.interrupts.ime = true;
            self.cpu.interrupts.enable_ime_next = false;
        }

        // self.cpu.print_gameboy_doc_output(&mut self.memory);
        self.cpu.handle_serial_for_test_rom(&mut self.memory);
    }
    
}

fn main() {
    let mut gameboy = GameBoy::new();

    gameboy.memory.load_test_rom();
    gameboy.cpu.set_varibles_for_gb_doc();

    gameboy.cpu.print_gameboy_doc_output(&mut gameboy.memory);

    let mut timer = Timer::new(&mut gameboy.memory);

        for _ in 0..10000000 {
            // gameboy.cpu.print_gameboy_doc_output(&mut gameboy.memory);
            // Emulation loop (one step for now)
            
            gameboy.step();
            timer.step(gameboy.cpu.cycles, &mut gameboy.memory);
            gameboy.cpu.cycles = 0;

            if gameboy.cpu.is_stopped {
                println!("Stopped on {}", gameboy.cpu.pc);
            //     break;
            }
        }



    println!("fin");

}
