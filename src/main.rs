use std::io::{self, Write};
use std::process;

use rustboy::{run_test_rom, TestOutcome};

fn main() {
    // ROM path from the first CLI arg, or default to the interrupts test.
    let rom_path = std::env::args().nth(1).unwrap_or_else(|| {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/roms/gb-test-roms/cpu_instrs/individual/02-interrupts.gb"
        )
        .to_string()
    });

    println!("Running {rom_path}");

    let run = match run_test_rom(&rom_path, |byte| {
        print!("{}", byte as char);
        io::stdout().flush().ok();
    }) {
        Ok(run) => run,
        Err(err) => {
            eprintln!("failed to run {rom_path}: {err}");
            process::exit(2);
        }
    };

    println!();
    match run.outcome {
        TestOutcome::Passed => process::exit(0),
        TestOutcome::Failed => process::exit(1),
        TestOutcome::Stuck { pc } => {
            eprintln!("[emulator] stuck at PC {pc:#06X} — no verdict from ROM");
            process::exit(2);
        }
    }
}
