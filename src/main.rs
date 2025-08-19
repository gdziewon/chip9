use chip9::Chip9;
use chip9::Emulator;
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

    let mut chip9 = Chip9::new();
    chip9.load_program(program).unwrap();

    let mut app = Emulator::new();

    if let Err(e) = app.run(chip9) {
        eprintln!("Error while running chip9: {e}");
    }
}
