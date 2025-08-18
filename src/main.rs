use chip8::Chip8;
use chip8::Emulator;
use std::fs::File;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: PathBuf
}

fn main() {
    let args = Args::parse();
    let program = &File::open(args.path).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_program(program).unwrap();

    let mut app = Emulator::new();

    if let Err(e) = app.run(chip8) {
        eprintln!("Error while running chip8: {e}");
    }
}
