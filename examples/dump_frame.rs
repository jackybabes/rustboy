//! Dump a frame from a ROM to a PGM image: `cargo run --example dump_frame -- <rom> [frames] [out.pgm]`
use std::io::Write;

fn main() {
    let mut args = std::env::args().skip(1);
    let rom = args.next().expect("usage: dump_frame <rom> [frames] [out.pgm]");
    let frames: usize = args.next().map(|s| s.parse().unwrap()).unwrap_or(120);
    let out = args.next().unwrap_or_else(|| "frame.pgm".into());

    let mut gb = rustboy::GameBoy::new();
    gb.load_rom(std::fs::read(&rom).unwrap());
    rustboy::gameboy_doctor::gb_doc_set_inital_registers(&mut gb.cpu);

    for _ in 0..frames {
        gb.run_frame();
    }

    let fb = gb.framebuffer();
    let map = [255u8, 170, 85, 0]; // shade 0..3 -> grey
    let mut f = std::fs::File::create(&out).unwrap();
    write!(f, "P5\n160 144\n255\n").unwrap();
    let bytes: Vec<u8> = fb.iter().map(|&s| map[s as usize]).collect();
    f.write_all(&bytes).unwrap();
    eprintln!("wrote {out} after {frames} frames");
}
