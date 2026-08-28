use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

use minifb::{Key, Scale, Window, WindowOptions};
use rustboy::gameboy_doctor::gb_doc_set_inital_registers;
use rustboy::joypad::Button;
use rustboy::ppu::{SCREEN_H, SCREEN_W};
use rustboy::{run_test_rom, GameBoy, TestOutcome};

const DEFAULT_ROM: &str = "roms/gb-test-roms/cpu_instrs/cpu_instrs.gb";

/// DMG shade (0..3) -> 0RGB, a soft green LCD look.
const SHADES: [u32; 4] = [0x00E0F8D0, 0x0088C070, 0x00346856, 0x00081820];

const KEYMAP: &[(Key, Button)] = &[
    (Key::Right, Button::Right),
    (Key::Left, Button::Left),
    (Key::Up, Button::Up),
    (Key::Down, Button::Down),
    (Key::Z, Button::A),
    (Key::X, Button::B),
    (Key::Enter, Button::Start),
    (Key::RightShift, Button::Select),
    (Key::Backspace, Button::Select),
];

fn main() {
    let mut headless = false;
    let mut rom_arg: Option<String> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--headless" | "-H" => headless = true,
            _ => rom_arg = Some(arg),
        }
    }

    let rom_path = match resolve_rom(rom_arg.as_deref()) {
        Some(p) => p,
        None => {
            eprintln!(
                "ROM not found: {}\nusage: cargo run -- [--headless] <path/to/rom.gb>",
                rom_arg.as_deref().unwrap_or(DEFAULT_ROM)
            );
            process::exit(2);
        }
    };

    if headless {
        run_headless(&rom_path);
    } else {
        run_windowed(&rom_path);
    }
}

/// Headless: run a blargg-style test ROM and report its serial verdict.
fn run_headless(rom_path: &Path) -> ! {
    println!("Running {} (headless)", rom_path.display());
    let run = match run_test_rom(rom_path, |b| {
        print!("{}", b as char);
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

/// Windowed: run the emulator at 60 fps and show the LCD.
fn run_windowed(rom_path: &Path) -> ! {
    let rom = std::fs::read(rom_path).unwrap_or_else(|e| {
        eprintln!("failed to read {}: {e}", rom_path.display());
        process::exit(2);
    });

    let mut gb = GameBoy::new();
    gb.load_rom(rom);
    gb_doc_set_inital_registers(&mut gb.cpu);

    let mut window = Window::new(
        &format!("rustboy — {}", rom_path.file_name().unwrap().to_string_lossy()),
        SCREEN_W,
        SCREEN_H,
        WindowOptions { scale: Scale::X4, ..WindowOptions::default() },
    )
    .unwrap_or_else(|e| {
        eprintln!("failed to open window: {e}");
        process::exit(2);
    });
    window.set_target_fps(60);

    let mut buf = vec![0u32; SCREEN_W * SCREEN_H];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for &(key, button) in KEYMAP {
            gb.memory.joypad.set(button, window.is_key_down(key));
        }

        gb.run_frame();

        for (dst, &shade) in buf.iter_mut().zip(gb.framebuffer()) {
            *dst = SHADES[shade as usize];
        }
        window
            .update_with_buffer(&buf, SCREEN_W, SCREEN_H)
            .unwrap_or_else(|e| {
                eprintln!("window update failed: {e}");
                process::exit(2);
            });
    }

    process::exit(0);
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