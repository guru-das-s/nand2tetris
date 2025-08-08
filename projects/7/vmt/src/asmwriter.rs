use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::spec::*;

pub struct AsmWriter<'a> {
    vmcmds: Vec<VmCommand>,
    file: &'a mut BufWriter<File>,
    filename: &'a str,
}
impl<'a> AsmWriter<'a> {
    pub fn new(cmds: Vec<VmCommand>, file: &'a mut BufWriter<File>, filename: &'a str) -> Self {
        Self {
            vmcmds: cmds,
            file: file,
            filename: filename,
        }
    }

    pub fn write(&mut self) -> Result<(), Box<dyn Error>> {
        let mut inside_a_function: Option<Arg1> = None;

        for vmcmd in &self.vmcmds {
            if vmcmd.is_function() {
                inside_a_function = vmcmd.arg1.clone();
            }

            let mut asm_code = vmcmd.code(&inside_a_function)?;

            // If a phrase has "FILE" placeholder in it, replace it
            asm_code = asm_code.replace("FILE", self.filename);

            writeln!(self.file, "{}", asm_code)?;
        }
        Ok(())
    }
}
