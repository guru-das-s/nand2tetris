use std::sync::{Mutex, OnceLock};

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
        static E: OnceLock<Mutex<u16>> = OnceLock::new();
        static G: OnceLock<Mutex<u16>> = OnceLock::new();
        static L: OnceLock<Mutex<u16>> = OnceLock::new();

        let e = E.get_or_init(|| Mutex::new(0));
        let g = G.get_or_init(|| Mutex::new(0));
        let l = L.get_or_init(|| Mutex::new(0));

        let mut eq = e.lock().unwrap();
        let mut gt = g.lock().unwrap();
        let mut lt = l.lock().unwrap();

        match self {
            Self::Add => Ok(phrases::ADD.to_string()),
            Self::Sub => Ok(phrases::SUB.to_string()),
            Self::Neg => Ok(phrases::NEG.to_string()),
            Self::Eq => {
                let s = phrases::EQ
                    .to_string()
                    .replace("XYZ", format!("{}", eq).as_str());
                *eq += 1;
                Ok(s)
            }
            Self::Gt => {
                let s = phrases::GT
                    .to_string()
                    .replace("XYZ", format!("{}", gt).as_str());
                *gt += 1;
                Ok(s)
            }
            Self::Lt => {
                let s = phrases::LT
                    .to_string()
                    .replace("XYZ", format!("{}", lt).as_str());
                *lt += 1;
                Ok(s)
            }
            Self::And => Ok(phrases::AND.to_string()),
            Self::Or => Ok(phrases::OR.to_string()),
            Self::Not => Ok(phrases::NOT.to_string()),
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VmCmdType {
    Push,
    Pop,
    Arithmetic(ArithmeticType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arg1 {
    Segment(Segment),
    Symbol(String),
}

impl Arg1 {
    pub fn max_limit(&self) -> Option<u16> {
        match self {
            Arg1::Segment(s) => s.max_limit(),
            Arg1::Symbol(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VmCommand {
    pub cmd: VmCmdType,
    pub arg1: Option<Arg1>,
    pub arg2: Option<u16>,
}
impl VmCommand {
    pub fn new(cmd: VmCmdType, arg1: Option<Arg1>, arg2: Option<u16>) -> Self {
        Self { cmd, arg1, arg2 }
    }

    pub fn segment_to_phrase(&self, segment: &Segment) -> Result<String, String> {
        let cmd = self.cmd;

        match cmd {
            VmCmdType::Push => {
                match segment {
                    Segment::Constant => Ok(phrases::CONSTANT.to_string()),
                    Segment::Local => Ok(phrases::SEGMENT.replace("SEG", "LCL").to_string()),
                    Segment::Argument => Ok(phrases::SEGMENT.replace("SEG", "ARG").to_string()),
                    Segment::This => Ok(phrases::SEGMENT.replace("SEG", "THIS").to_string()),
                    Segment::That => Ok(phrases::SEGMENT.replace("SEG", "THAT").to_string()),
                    Segment::Static => Ok(phrases::STATIC.to_string()), // upper layers will handle str replacement
                    Segment::Temp => Ok(phrases::TEMP.to_string()),
                    Segment::Pointer => Ok(phrases::POINTER.to_string()),
                }
            }
            VmCmdType::Pop => {
                match segment {
                    Segment::Constant => Err(format!("Cannot pop to constant segment")),
                    Segment::Local => {
                        Ok(phrases::SEGMENT_ADDRESS.replace("SEG", "LCL").to_string())
                    }
                    Segment::Argument => {
                        Ok(phrases::SEGMENT_ADDRESS.replace("SEG", "ARG").to_string())
                    }
                    Segment::This => {
                        Ok(phrases::SEGMENT_ADDRESS.replace("SEG", "THIS").to_string())
                    }
                    Segment::That => {
                        Ok(phrases::SEGMENT_ADDRESS.replace("SEG", "THAT").to_string())
                    }
                    Segment::Static => Ok(phrases::STATIC_ADDRESS.to_string()), // upper layers will handle str replacement
                    Segment::Temp => Ok(phrases::TEMP_ADDRESS.to_string()),
                    Segment::Pointer => Ok(phrases::POINTER_ADDRESS.to_string()),
                }
            }
            _ => Err(format!(
                "Command {:?} does not need segment/i computation",
                cmd
            )),
        }
    }

    pub fn to_phrase(&self, arg1: &Arg1) -> Result<String, String> {
        match arg1 {
            Arg1::Segment(s) => self.segment_to_phrase(s),
            Arg1::Symbol(_) => todo!(),
        }
    }

    fn code_segment_i(&self, s: Segment, i: u16, phrase: &str) -> Result<String, String> {
        let code: String = match s {
            Segment::Pointer => {
                let tot = if i == 0 { "THIS" } else { "THAT" };
                phrase.replace("XYZ", tot)
            }
            _ => phrase.replace("XYZ", format!("{}", i).as_str()),
        };
        Ok(code)
    }

    fn code_segment(self) -> Result<String, String> {
        let arg1 = self
            .arg1
            .ok_or(format!("Push/Pop command segment cannot be empty"))?;

        let i = self
            .arg2
            .ok_or(format!("Push/Pop command arg2 cannot be empty"))?;

        let phrase = self.to_phrase(&arg1)?;
        self.code_segment_i(arg1, i, &phrase)
    }

    fn code_push(&self) -> Result<String, String> {
        let seg_code = self.code_segment()?;
        Ok(seg_code + phrases::PUSH)
    }

    fn code_pop(&self) -> Result<String, String> {
        let seg_code = self.code_segment()?;
        Ok(phrases::POP_PRE.to_string() + &seg_code + phrases::POP)
    }

    pub fn code(&self) -> Result<String, String> {
        match self.cmd {
            VmCmdType::Push => self.code_push(),
            VmCmdType::Pop => self.code_pop(),
            VmCmdType::Arithmetic(op) => op.code(),
        }
    }
}
pub const MAX_NUM_VM_CMD_PARTS: usize = 3;
