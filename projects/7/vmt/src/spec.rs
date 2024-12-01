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

    pub fn to_phrase(&self) -> Result<String, String> {
        match self {
            Segment::Constant => Ok(phrases::CONSTANT.to_string()),
            Segment::Local => Ok(phrases::SEGMENT.replace("SEG", "LCL").to_string()),
            Segment::Static => Ok(phrases::STATIC.to_string()),
            Segment::Temp => Ok(phrases::SEGMENT.replace("SEG", "5").to_string()),
            _ => Err(format!("Phrase not implemented for {:?}", self)),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VmCmdType {
    Push,
    Pop,
    Arithmetic(ArithmeticType),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VmCommand {
    pub cmd: VmCmdType,
    pub arg1: Option<Segment>,
    pub arg2: Option<u16>,
}
impl VmCommand {
    pub fn new(cmd: VmCmdType, arg1: Option<Segment>, arg2: Option<u16>) -> Self {
        Self { cmd, arg1, arg2 }
    }

    fn code_segment_i(&self, phrase: String) -> Result<String, String> {
        let i = self
            .arg2
            .ok_or(format!("Push command arg2 cannot be empty"))?;

        let s = phrase.replace("XYZ", format!("{}", i).as_str());
        Ok(s)
    }

    fn code_segment(self) -> Result<String, String> {
        let segment = self
            .arg1
            .ok_or(format!("Push command segment cannot be empty"))?;
        let phrase = segment.to_phrase()?;
        self.code_segment_i(phrase)
    }

    fn code_push(&self) -> Result<String, String> {
        let seg_code = self.code_segment()?;
        Ok(seg_code + phrases::PUSH)
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
