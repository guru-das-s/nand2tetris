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
    let mut instr_num: u16 = 0;
    let mut line_iter = lines.iter().peekable();
    let mut label_found = false;
    let mut labels = Vec::<String>::new();

    while let Some(line) = line_iter.peek() {
        let parsed_line = HackLine::parse_line(&line)?;
        #[cfg(debug_assertions)]
        println!("[{}] {} -> {:?}", instr_num, line, parsed_line);
        match parsed_line {
            HackLine::A { .. } | HackLine::C { .. } | HackLine::Variable { .. } => {
                if label_found {
                    label_found = false;
                    // The multiple labels listed on consecutive lines need to
                    // be all mapped to the same instruction number, so pop each
                    // one of them and add them to the symbol table.
                    while let Some(label) = labels.pop() {
                        #[cfg(debug_assertions)]
                        println!("[{}] Adding label {} = {}", instr_num, label, instr_num);
                        symbol_table.add_new_label(label.clone(), instr_num);
                    }
                }
                instr_num += 1;
            }
            HackLine::Label { label, .. } => {
                label_found = true;
                // There may be multiple labels listed on consecutive lines, so
                // collect all of them.
                labels.push(label);
            }
            _ => {}
        }
        line_iter.next(); // advance the iterator
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
