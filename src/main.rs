mod cpu;
mod memory;
mod data;
mod interrupts;
mod timer;
mod gameboy_doctor;

use std::io::{self, Write};
use std::process;

use cpu::CPU;
use memory::Memory;
use data::HardwareRegister;
use interrupts::handle_interrupt;
use timer::Timer;

const DOTS_PER_FRAME: u32 = 70224;

pub struct GameBoy {
    pub cpu: CPU,
    pub memory: Memory,
    pub timer: Timer,
    ppu_dots: u32,
}

impl GameBoy {
    pub fn new() -> Self {
        let mut memory = Memory::new();
        let timer = Timer::new(&mut memory);

        GameBoy {
            cpu: CPU::new(),
            memory: memory,
            timer: timer,
            ppu_dots: 0,
        }
    }

    fn tick(&mut self, cycles: u16) {
        self.timer.step(cycles, &mut self.memory);

        // No PPU yet: fake a VBlank interrupt once per frame so ROMs that idle
        // in `EI; HALT` waiting for VBlank keep advancing.
        self.ppu_dots += cycles as u32;
        if self.ppu_dots >= DOTS_PER_FRAME {
            self.ppu_dots -= DOTS_PER_FRAME;
            let if_ = self.memory.read_hardware_register(HardwareRegister::IF);
            self.memory.write_hardware_register(HardwareRegister::IF, if_ | 0b0000_0001);
        }
    }

    fn step(&mut self) -> u16 {
        // 2. HALT: if halted, either wake on interrupt (HALT bug) or burn cycles
        if self.cpu.is_halted {
            let ie = self.memory.read_hardware_register(HardwareRegister::IE);
            let if_ = self.memory.read_hardware_register(HardwareRegister::IF);

            if ie & if_ & 0x1F != 0 {
                // A pending interrupt wakes the CPU regardless of IME. If IME is
                // set, the interrupt block below services it this same step.
                self.cpu.is_halted = false;
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

    let rom_path = "/Users/jack/Code/rustboy/roms/gb-test-roms/cpu_instrs/cpu_instrs.gb";

    gameboy_doctor::gb_doc_load_test_rom(&mut gameboy.memory, rom_path);
    gameboy_doctor::gb_doc_set_inital_registers(&mut gameboy.cpu);
    println!("Initial Registers");
    gameboy_doctor::gb_doc_print(&mut gameboy.cpu, &mut gameboy.memory);

    let mut serial = String::new();
    let mut last_pc = 0u16;
    let mut stuck = 0u32;

    loop {
        gameboy.step();

        if let Some(byte) = gameboy_doctor::gb_doc_handle_serial(&mut gameboy.memory) {
            print!("{}", byte as char);
            io::stdout().flush().ok();
            serial.push(byte as char);

            // blargg test ROMs report their verdict over serial, then spin.
            if serial.contains("Passed") {
                println!();
                process::exit(0);
            }
            if serial.contains("Failed") {
                println!();
                process::exit(1);
            }
        }

        // Safety net: bail if the CPU is genuinely wedged (PC frozen while not
        // halted) so a crashed ROM doesn't hang forever.
        if gameboy.cpu.pc == last_pc && !gameboy.cpu.is_halted {
            stuck += 1;
            if stuck > 5_000_000 {
                eprintln!("\n[emulator] stuck at PC {:#06X} — aborting", gameboy.cpu.pc);
                process::exit(2);
            }
        } else {
            stuck = 0;
        }
        last_pc = gameboy.cpu.pc;
    }
}
