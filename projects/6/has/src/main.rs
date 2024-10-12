use clap::Parser;
use parser::HackLine;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::{Path, PathBuf};
use symboltable::SymbolTable;

mod parser;
mod spec;
mod symboltable;
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

fn assemble(
    writer: &mut BufWriter<File>,
    lines: &Vec<String>,
    symbol_table: &mut SymbolTable,
) -> Result<(), Box<dyn Error>> {
    let mut instr_num: i16 = -1;

    for line in lines {
        let parsed_line = HackLine::parse_line(&line)?;
        match parsed_line {
            HackLine::A { .. } | HackLine::C { .. } | HackLine::Variable { .. } => instr_num += 1,
            HackLine::Label { label } => symbol_table.add_new_label(label, instr_num),
            _ => {}
        }
    }

    for line in lines {
        let parsed_line = HackLine::parse_line(&line)?;
        #[cfg(debug_assertions)]
        println!("{} -> {:?}", line, parsed_line);
        if let Some(bincode) = to_binary::binary_of(parsed_line, symbol_table) {
            writeln!(writer, "{}", bincode)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    if args.output.is_none() {
        args.output = Some(args.filename.with_extension("hack"));
    }

    let mut symbol_table = SymbolTable::init();

    let output_file = File::create(args.output.as_ref().unwrap())?;
    let mut writer = BufWriter::new(output_file);

    let lines: Vec<_> = read_lines(args.filename)?.collect::<Result<_, _>>()?;

    if let Err(e) = assemble(&mut writer, &lines, &mut symbol_table) {
        std::fs::remove_file(args.output.unwrap())?;
        return Err(e);
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
