use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

use rustboy::{run_test_rom, TestOutcome};

const DEFAULT_ROM: &str = "roms/gb-test-roms/cpu_instrs/cpu_instrs.gb";

fn main() {
    // First CLI arg is the ROM path (absolute, or relative to the cwd or the
    // crate root). With no arg, fall back to the combined cpu_instrs ROM.
    let arg = std::env::args().nth(1);
    let rom_path = match resolve_rom(arg.as_deref()) {
        Some(path) => path,
        None => {
            eprintln!(
                "ROM not found: {}\nusage: cargo run -- <path/to/rom.gb>   (note the `--`)",
                arg.as_deref().unwrap_or(DEFAULT_ROM)
            );
            process::exit(2);
        }
    };

    println!("Running {}", rom_path.display());

    let run = match run_test_rom(&rom_path, |byte| {
        print!("{}", byte as char);
        io::stdout().flush().ok();
    }) {
        Ok(run) => run,
        Err(err) => {
            eprintln!("failed to run {}: {err}", rom_path.display());
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

/// Resolve a ROM path: try it as given, then relative to the crate root.
fn resolve_rom(arg: Option<&str>) -> Option<PathBuf> {
    let raw = arg.unwrap_or(DEFAULT_ROM);

    let as_given = PathBuf::from(raw);
    if as_given.is_file() {
        return Some(as_given);
    }

    let from_crate_root = Path::new(env!("CARGO_MANIFEST_DIR")).join(raw);
    if from_crate_root.is_file() {
        return Some(from_crate_root);
    }

    None
}
