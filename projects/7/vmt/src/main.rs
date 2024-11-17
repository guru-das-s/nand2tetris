use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufWriter};
use std::path::{Path, PathBuf};

mod asmwriter;
mod parser;
mod phrases;
mod spec;

#[derive(Default, Parser, Debug)]
/// Hack VM Translator
///
/// Converts Hack VM files to Hack assembly
#[command(version)]
struct Args {
    /// Path to Hack VM file (e.g. SimpleAdd.vm)
    #[arg(short, long)]
    filename: PathBuf,

    /// Output filename (e.g. SimpleAdd.asm)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    if args.output.is_none() {
        args.output = Some(args.filename.with_extension("asm"));
    }

    let output_file = File::create(args.output.as_ref().unwrap())?;
    let writer = BufWriter::new(output_file);

    let lines: Vec<_> = read_lines(args.filename)?.collect::<Result<_, _>>()?;

    let mut p = parser::Parser::new(lines.as_ref());
    p.parse()?;
    p.print_parsed();

    let w = asmwriter::AsmWriter::new(p.parsed, writer);
    w.write()?;

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
