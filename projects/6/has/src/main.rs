use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod parser;
mod spec;

#[derive(Parser, Debug)]
/// Hack assembler
///
/// Converts Hack assembly files to Hack binary
#[command(version)]
struct Args {
    /// Path to Hack ASM file (e.g. Max.asm)
    #[arg(short, long)]
    filename: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let lines = read_lines(args.filename)?;

    for line in lines.flatten() {
        let parsed_line = parser::HackLine::parse_line(&line)?;
        println!("{} -> {:?}", line, parsed_line);
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}