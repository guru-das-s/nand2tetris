use crate::phrases;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArithmeticType {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}
impl ArithmeticType {
    pub fn code(&self) -> Result<String, String> {
        match self {
            Self::Add => Ok(phrases::ADD.to_string()),
            _ => Err(format!(
                "asm for arithmetic operation {:?} not implemented yet",
                self
            )),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Segment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp,
}
impl Segment {
    pub const MAX_TEMP_VARS: u16 = 7;
    pub const MAX_STATIC_VARS: u16 = 240; // = 255 - 16 + 1
    pub const MAX_POINTER_VARS: u16 = 2;

    pub fn max_limit(&self) -> Option<u16> {
        match self {
            Segment::Temp => Some(Self::MAX_TEMP_VARS),
            Segment::Static => Some(Self::MAX_STATIC_VARS),
            Segment::Pointer => Some(Self::MAX_POINTER_VARS),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VmCmdType {
    Push,
    Pop,
    Arithmetic(ArithmeticType),
}

#[derive(Debug, PartialEq)]
pub struct VmCommand {
    pub cmd: VmCmdType,
    pub arg1: Option<Segment>,
    pub arg2: Option<u16>,
}
impl VmCommand {
    pub fn new(cmd: VmCmdType, arg1: Option<Segment>, arg2: Option<u16>) -> Self {
        Self { cmd, arg1, arg2 }
    }

    fn code_push_constant(&self) -> Result<String, String> {
        let i = self
            .arg2
            .ok_or(format!("Push command arg2 cannot be empty"))?;

        let s = phrases::PUSH_CONSTANT.replace("XYZ", format!("{}", i).as_str());
        Ok(s)
    }

    fn code_push(&self) -> Result<String, String> {
        let segment = self
            .arg1
            .ok_or(format!("Push command segment cannot be empty"))?;

        match segment {
            Segment::Constant => self.code_push_constant(),
            _ => Err(format!(
                "asm for push segment {:?} not implemented yet",
                segment
            )),
        }
    }

    pub fn code(&self) -> Result<String, String> {
        match self.cmd {
            VmCmdType::Push => self.code_push(),
            VmCmdType::Arithmetic(op) => op.code(),
            _ => Err(format!(
                "asm for VM command {:?} not implemented yet",
                self.cmd
            )),
        }
    }
}
pub const MAX_NUM_VM_CMD_PARTS: usize = 3;
