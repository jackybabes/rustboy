mod cpu;
use cpu::CPU;

mod memory;
use memory::Memory;


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

        // get next op code
        let opcode = self.cpu.fetch_byte(&self.memory);
        println!("0x{:02X}", opcode);
        
        // execute op code  
        self.cpu.execute(opcode, &mut self.memory);
        println!("{}", self.cpu);

        // print chars for blarg test roms
        self.cpu.handle_serial_for_test_rom(&mut self.memory);
    }
    
}

fn main() {
    let mut gameboy = GameBoy::new();

    gameboy.memory.load_test_rom();



    for _ in 0..100 {
        // Emulation loop (one step for now)
        gameboy.step();
    }



    println!("fin");

}
