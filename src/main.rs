mod cpu;
use cpu::CPU;

mod memory;
use memory::Memory;

mod data;
use data::HardwareRegister;

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

    fn handle_interrupt(&mut self, interrupt: u8) {
        let addr = match interrupt {
            0 => 0x40, // V-Blank
            1 => 0x48, // LCD STAT
            2 => 0x50, // Timer
            3 => 0x58, // Serial
            4 => 0x60, // Joypad
            _ => return,
        };

        // Push current PC to stack
        self.cpu.push_u16(&mut self.memory, self.cpu.pc);
        self.cpu.pc = addr;

        self.cpu.cycles += 20;
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

                        self.handle_interrupt(i);

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

        self.cpu.print_gameboy_doc_output(&mut self.memory);


    }
    
}

fn main() {
    let mut gameboy = GameBoy::new();

    gameboy.memory.load_test_rom();
    gameboy.cpu.set_varibles_for_gb_doc();

    gameboy.cpu.print_gameboy_doc_output(&mut gameboy.memory);

    for _ in 0..1000000 {
        // gameboy.cpu.print_gameboy_doc_output(&mut gameboy.memory);
        // Emulation loop (one step for now)
        gameboy.step();
        if gameboy.cpu.is_stopped {

            println!("Stopped on {}", gameboy.cpu.pc);
            break;
        }
    }



    println!("fin");

}
