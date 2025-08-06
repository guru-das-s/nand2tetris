use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
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
    /// Path to Hack VM file (e.g. SimpleAdd.vm) or directory
    #[arg(short, long)]
    input_path: PathBuf,

    /// Output filename (e.g. SimpleAdd.asm)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    let mut vm_files: Vec<PathBuf> = Vec::new();

    if args.input_path.is_dir() {
        vm_files = std::fs::read_dir(&args.input_path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file() && p.extension().map_or(false, |ext| ext == "vm"))
            .collect();
    } else {
        vm_files.push(args.input_path.clone());
    }

    if args.output.is_none() {
        args.output = if args.input_path.is_dir() {
            let dir_name = args
                .input_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Could not determine directory name");
            Some(args.input_path.join(format!("{}.asm", dir_name)))
        } else {
            Some(args.input_path.with_extension("asm"))
        }
    }

    let output_file = File::create(args.output.as_ref().unwrap())?;
    let mut writer = BufWriter::new(output_file);

    if args.input_path.is_dir() {
        writeln!(writer, "{}", phrases::BOOTSTRAP)?;
    }

    for vm_file in vm_files {
        let lines: Vec<_> = read_lines(&vm_file)?.collect::<Result<_, _>>()?;

        let mut p = parser::Parser::new(lines.as_ref());
        p.parse()?;
        p.print_parsed();

        let mut w = asmwriter::AsmWriter::new(
            p.parsed,
            &mut writer,
            vm_file.file_stem().unwrap().to_str().unwrap(),
        );
        w.write()?;
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
