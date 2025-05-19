use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::spec::*;

pub struct AsmWriter<'a> {
    vmcmds: Vec<VmCommand>,
    file: BufWriter<File>,
    filename: &'a str,
}
impl<'a> AsmWriter<'a> {
    pub fn new(cmds: Vec<VmCommand>, file: BufWriter<File>, filename: &'a str) -> Self {
        Self {
            vmcmds: cmds,
            file: file,
            filename: filename,
        }
    }

    pub fn write(mut self) -> Result<(), Box<dyn Error>> {
        for vmcmd in self.vmcmds {
            let mut asm_code = vmcmd.code()?;

            asm_code = if vmcmd
                .arg1
                .is_some_and(|segment| segment == Arg1::Segment(Segment::Static))
            {
                asm_code.replace("FILE", self.filename)
            } else {
                asm_code
            };

            writeln!(self.file, "{}", asm_code)?;
        }
        Ok(())
    }
}
