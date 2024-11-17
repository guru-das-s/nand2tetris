use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::spec::*;

pub struct AsmWriter {
    vmcmds: Vec<VmCommand>,
    file: BufWriter<File>,
}
impl AsmWriter {
    pub fn new(cmds: Vec<VmCommand>, file: BufWriter<File>) -> Self {
        Self {
            vmcmds: cmds,
            file: file,
        }
    }

    pub fn write(mut self) -> Result<(), Box<dyn Error>> {
        for vmcmd in self.vmcmds {
            let asm_code = vmcmd.code()?;
            writeln!(self.file, "{}", asm_code)?;
        }
        Ok(())
    }
}
