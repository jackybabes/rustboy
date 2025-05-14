mod cpu;
mod memory;
mod data;
mod interrupts;
mod timer;
mod gameboy_doctor;

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

    pub fn step(&mut self) -> u16 {
        // handle stop
        if self.cpu.is_stopped {
            let joypad = self.memory.read_hardware_register(HardwareRegister::P1);
            if joypad & 0x0F != 0x0F {
                self.cpu.is_stopped = false;
            } else {
                return 0;
            }
        }

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
                        return 20;
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
                return 4;
            }
        }

        // get next op code this increments the pc by 1
        let opcode = self.cpu.fetch_byte(&self.memory);
        // execute op code  
        let cycles = self.cpu.execute(opcode, &mut self.memory);


        // delayed enable interuopt
        if self.cpu.interrupts.enable_ime_next {
            self.cpu.interrupts.ime = true;
            self.cpu.interrupts.enable_ime_next = false;
        }
        
        // self.cpu.print_gameboy_doc_output(&mut self.memory);
        return cycles;


    }
    
}

fn main() {
    let mut gameboy = GameBoy::new();

    gameboy_doctor::gb_doc_load_test_rom(&mut gameboy.memory);
    gameboy_doctor::gb_doc_set_varibles(&mut gameboy.cpu);
    gameboy_doctor::gb_doc_print(&mut gameboy.cpu, &mut gameboy.memory);

    let mut timer = Timer::new(&mut gameboy.memory);

    const CYCLES_PER_FRAME: u32 = 70224;

    for _frame in 0..10000 {
        let mut cycles_this_frame = 0;

        while cycles_this_frame < CYCLES_PER_FRAME {
            let used_cycles = gameboy.step();
            if gameboy.cpu.is_stopped {
                continue;
            }

            timer.step(used_cycles, &mut gameboy.memory);
            cycles_this_frame += used_cycles as u32;

            gameboy_doctor::gb_doc_handle_serial(&mut gameboy.memory);
        }

        // Optionally: render frame, wait, print debug info, etc.
    }



    println!("fin");

}
