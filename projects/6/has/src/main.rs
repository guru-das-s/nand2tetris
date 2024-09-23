use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::{Path, PathBuf};

mod parser;
mod spec;
mod to_binary;

#[derive(Default, Parser, Debug)]
/// Hack assembler
///
/// Converts Hack assembly files to Hack binary
#[command(version)]
struct Args {
    /// Path to Hack ASM file (e.g. Max.asm)
    #[arg(short, long)]
    filename: PathBuf,

    /// Output filename (e.g. Max.hack)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    if args.output.is_none() {
        args.output = Some(args.filename.with_extension("hack"));
    }

    let output_file = File::create(args.output.unwrap())?;
    let mut writer = BufWriter::new(output_file);

    let lines = read_lines(args.filename)?;

    for line in lines.flatten() {
        let parsed_line = parser::HackLine::parse_line(&line)?;
        println!("{} -> {:?}", line, parsed_line);
        if let Some(bincode) = to_binary::binary_of(parsed_line) {
            writeln!(writer, "{}", bincode)?;
        }
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
