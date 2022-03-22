mod cpu;
mod registers;
mod rom;

use cpu::{CPU, Instruction};
use rom::*;

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the rom file to load
    #[clap(short, long)]
    rom: String,
}

fn parse_rom(rom_bytes: &[u8]) -> Rom {
    Rom {
        info_header: InfoHeader {

        }
    }
}

fn main() {
    let args = Args::parse();

    let rom_bytes = fs::read(args.rom).expect("Invalid rom path. Exiting.");

    println!("Loading rom: {}", args.rom);

    parse_rom(&rom_bytes);

    let cpu: CPU = CPU::new();
    println!("cpu: {}", cpu);
}
