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
            // if vmcmd.is_function()
            //      save its Arg1 to `function_seen`, an Option<Symbol>
            if vmcmd.is_function() {
                inside_a_function = vmcmd.arg1.clone();
            }

            // If we see a LABEL when `function_seen` is true, use phrase LABEL_IN_FUNC
            //      ... I guess we need to pass `function_seen` as param to vmcmd.code(...)
            // ^^^ is done now

            // If we see a call, replace its `FUNC` phrase field with `function_seen`'s Symbol(String) function name
            // ^^^ is done now

            let mut asm_code = vmcmd.code(&inside_a_function)?;

            asm_code = if vmcmd
                .arg1
                .clone()
                .is_some_and(|segment| segment == Arg1::Segment(Segment::Static))
                || vmcmd.is_destination_cmd()
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
