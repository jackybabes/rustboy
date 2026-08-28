pub mod cpu;
pub mod memory;
pub mod data;
pub mod interrupts;
pub mod timer;
pub mod gameboy_doctor;

use std::io;
use std::path::Path;

use cpu::CPU;
use memory::Memory;
use data::HardwareRegister;
use interrupts::handle_interrupt;
use timer::Timer;

const DOTS_PER_FRAME: u32 = 70224;

/// How many consecutive steps with a frozen PC (while not halted) counts as a
/// wedged CPU rather than a legitimate wait loop.
const STUCK_LIMIT: u32 = 5_000_000;

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
            memory,
            timer,
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

    pub fn step(&mut self) -> u16 {
        // HALT: if halted, wake on a pending interrupt or burn cycles.
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

        // Interrupts: if IME set and any enabled interrupt is pending, take one.
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

        // Delayed IME (EI takes effect after the next instruction).
        if self.cpu.interrupts.enable_ime_next {
            self.cpu.interrupts.ime = true;
            self.cpu.interrupts.enable_ime_next = false;
        }

        // Execute one instruction.
        let opcode = self.cpu.fetch_byte(&self.memory);
        let cycles = self.cpu.execute(opcode, &mut self.memory);

        self.tick(cycles);
        cycles
    }

    /// Load a 32 KiB (non-banked) ROM image into the ROM region.
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, byte) in rom.iter().take(0x8000).enumerate() {
            self.memory.write_byte(i as u16, *byte);
        }
    }
}

impl Default for GameBoy {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of running a blargg-style test ROM to completion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestOutcome {
    Passed,
    Failed,
    /// CPU wedged (PC frozen while not halted) before reporting a verdict.
    Stuck { pc: u16 },
}

pub struct TestRun {
    pub outcome: TestOutcome,
    /// Everything the ROM wrote to the serial port.
    pub serial: String,
}

/// Run a blargg cpu_instrs-style test ROM headlessly until it reports "Passed"
/// or "Failed" over the serial port, or the CPU wedges.
///
/// `on_serial` is invoked for every serial byte so callers can stream output.
pub fn run_test_rom(
    path: impl AsRef<Path>,
    mut on_serial: impl FnMut(u8),
) -> io::Result<TestRun> {
    let rom = std::fs::read(path.as_ref())?;

    let mut gb = GameBoy::new();
    gb.load_rom(&rom);
    gameboy_doctor::gb_doc_set_inital_registers(&mut gb.cpu);

    let mut serial = String::new();
    let mut last_pc = 0u16;
    let mut stuck = 0u32;

    loop {
        gb.step();

        if let Some(byte) = gameboy_doctor::gb_doc_handle_serial(&mut gb.memory) {
            on_serial(byte);
            serial.push(byte as char);

            if serial.contains("Passed") {
                return Ok(TestRun { outcome: TestOutcome::Passed, serial });
            }
            if serial.contains("Failed") {
                return Ok(TestRun { outcome: TestOutcome::Failed, serial });
            }
        }

        if gb.cpu.pc == last_pc && !gb.cpu.is_halted {
            stuck += 1;
            if stuck > STUCK_LIMIT {
                return Ok(TestRun {
                    outcome: TestOutcome::Stuck { pc: gb.cpu.pc },
                    serial,
                });
            }
        } else {
            stuck = 0;
        }
        last_pc = gb.cpu.pc;
    }
}
