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
    pub timer: Timer,
}

impl GameBoy {
    pub fn new() -> Self {
        let mut memory = Memory::new();
        let timer = Timer::new(&mut memory);

        GameBoy {
            cpu: CPU::new(),
            memory: memory,
            timer: timer,
        }
    }

    fn tick(&mut self, cycles: u16) {
        self.timer.step(cycles, &mut self.memory);
        // ppu etc
    }

    fn step(&mut self) -> u16 {
        // 2. HALT: if halted, either wake on interrupt (HALT bug) or burn cycles
        if self.cpu.is_halted {
            // print!("Halted");
            let ie = self.memory.read_hardware_register(HardwareRegister::IE);
            let if_ = self.memory.read_hardware_register(HardwareRegister::IF);
            let pending = ie & if_;

            if !self.cpu.interrupts.ime && pending != 0 {
                self.cpu.is_halted = false; // HALT bug: wake but don't jump yet
            } else if self.cpu.interrupts.ime && pending != 0 {
                self.cpu.is_halted = false; // normal: next step will take interrupt
            } else {
                self.tick(4);
                return 4;
            }
        }

        // 1. Interrupts: if IME set and any enabled interrupt pending, take one
        if self.cpu.interrupts.ime {
            let ie = self.memory.read_hardware_register(HardwareRegister::IE);
            let mut if_ = self.memory.read_hardware_register(HardwareRegister::IF);
            let pending = ie & if_;

            if pending != 0 {
                let i = (0..5).find(|&i| (pending & (1 << i)) != 0).unwrap();
                if_ &= !(1 << i);
                self.memory.write_hardware_register(HardwareRegister::IF, if_);

                handle_interrupt(&mut self.cpu, &mut self.memory, i);
                self.cpu.interrupts.ime = false;
                self.tick(20);
                return 20;
            }
        }


        // Delayed IME (EI takes effect after next instruction)
        if self.cpu.interrupts.enable_ime_next {
            self.cpu.interrupts.ime = true;
            self.cpu.interrupts.enable_ime_next = false;
        }

        // 3. Execute one instruction
        let opcode = self.cpu.fetch_byte(&self.memory);
        let cycles = self.cpu.execute(opcode, &mut self.memory);

        self.tick(cycles);
        cycles
    }
    
}

fn main() {
    let mut gameboy = GameBoy::new();

    let rom_path = "/Users/jack/Code/rustboy/roms/gb-test-roms/cpu_instrs/individual/02-interrupts.gb";

    gameboy_doctor::gb_doc_load_test_rom(&mut gameboy.memory, rom_path);
    gameboy_doctor::gb_doc_set_inital_registers(&mut gameboy.cpu);
    println!("Initial Registers");
    gameboy_doctor::gb_doc_print(&mut gameboy.cpu, &mut gameboy.memory);

    let mut last_pc = 0;
    let mut stable_count = 0; 


    loop {
        gameboy.step();
        // You can add any logging/printing here if desired, e.g. println!("Cycles: {}", cycles);
        gameboy_doctor::gb_doc_handle_serial(&mut gameboy.memory);
        // println!("{}", gameboy.cpu);

        // Test for infinite loop
        if last_pc == gameboy.cpu.pc {
            stable_count += 1;
            if stable_count > 10000  {
                println!("Stable count: {}", stable_count);
                break;
            }
        } else {
            stable_count = 0;
        }

        last_pc = gameboy.cpu.pc;
    }
}
