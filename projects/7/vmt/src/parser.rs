use std::{iter::Peekable, slice::Iter};

use crate::spec::*;

pub struct Parser<'a> {
    li: Peekable<Iter<'a, String>>,
    pub parsed: Vec<VmCommand>,
}

impl<'a> Parser<'a> {
    pub fn new(lines: &'a Vec<String>) -> Self {
        Parser {
            li: lines.iter().peekable(),
            parsed: Vec::<VmCommand>::new(),
        }
    }

    fn current_line(&mut self) -> Option<String> {
        self.li.peek().map(|s| (*s).clone())
    }

    fn advance(&mut self) {
        self.li.next();
    }

    fn parse_line_cmd_type(&self, word: &str) -> Result<VmCmdType, String> {
        match word {
            "push" => Ok(VmCmdType::Push),
            "pop" => Ok(VmCmdType::Pop),
            "add" => Ok(VmCmdType::Arithmetic(ArithmeticType::Add)),
            "sub" => Ok(VmCmdType::Arithmetic(ArithmeticType::Sub)),
            "neg" => Ok(VmCmdType::Arithmetic(ArithmeticType::Neg)),
            "eq" => Ok(VmCmdType::Arithmetic(ArithmeticType::Eq)),
            "gt" => Ok(VmCmdType::Arithmetic(ArithmeticType::Gt)),
            "lt" => Ok(VmCmdType::Arithmetic(ArithmeticType::Lt)),
            "and" => Ok(VmCmdType::Arithmetic(ArithmeticType::And)),
            "or" => Ok(VmCmdType::Arithmetic(ArithmeticType::Or)),
            "not" => Ok(VmCmdType::Arithmetic(ArithmeticType::Not)),
            _ => Err(format!("Invalid VM command type: {}", word)),
        }
    }

    fn parse_arg1(&self, word: &str) -> Result<Segment, String> {
        match word {
            "local" => Ok(Segment::Local),
            "argument" => Ok(Segment::Argument),
            "this" => Ok(Segment::This),
            "that" => Ok(Segment::That),
            "constant" => Ok(Segment::Constant),
            "static" => Ok(Segment::Static),
            "pointer" => Ok(Segment::Pointer),
            "temp" => Ok(Segment::Temp),
            _ => Err(format!("Invalid segment: {}", word)),
        }
    }

    fn parse_i(&self, word: &str) -> Result<u16, String> {
        let i_or_err = word.parse::<u16>();
        match i_or_err {
            Err(e) => return Err(format!("{}: {}", e.to_string(), word)),
            Ok(i) => Ok(i),
        }
    }

    fn parse_arg2(&self, s: &Segment, word: &str) -> Result<Option<u16>, String> {
        let i = self.parse_i(word)?;
        if let Some(max_limit) = s.max_limit() {
            if i > max_limit {
                return Err(format!("i for segment {:?} cannot exceed {}", s, max_limit));
            } else {
                return Ok(Some(i));
            }
        } else {
            return Ok(Some(i));
        }
    }

    fn parse_line_arg1(&self, t: &VmCmdType, parts: &Vec<&str>) -> Result<Option<Segment>, String> {
        match t {
            VmCmdType::Push | VmCmdType::Pop => Ok(Some(self.parse_arg1(parts[1])?)),
            VmCmdType::Arithmetic(..) => Ok(None),
        }
    }

    fn parse_line_arg2(
        &self,
        s: &Option<Segment>,
        parts: &Vec<&str>,
    ) -> Result<Option<u16>, String> {
        match s {
            None => return Ok(None),
            Some(segment) => return self.parse_arg2(segment, parts[2]),
        }
    }

    fn parse_line(&mut self, line: String) -> Result<Option<VmCommand>, String> {
        if line.trim().is_empty() || line.trim().starts_with("//") {
            return Ok(None);
        }

        let parts: Vec<&str> = line.trim().split(' ').collect();

        if parts.len() > MAX_NUM_VM_CMD_PARTS {
            return Err(format!(
                "Line: '{}' exceeds max {} number of parts",
                line, MAX_NUM_VM_CMD_PARTS
            ));
        }

        let cmd_type = self.parse_line_cmd_type(parts[0])?;
        let segment = self.parse_line_arg1(&cmd_type, &parts)?;
        let i = self.parse_line_arg2(&segment, &parts)?;

        Ok(Some(VmCommand::new(cmd_type, segment, i)))
    }

    pub fn parse(&mut self) -> Result<(), String> {
        while let Some(line) = self.current_line() {
            if let Some(cmd) = self.parse_line(line)? {
                self.parsed.push(cmd);
            }
            self.advance();
        }
        Ok(())
    }

    pub fn print_parsed(&self) {
        #[cfg(debug_assertions)]
        println!("{:#?}", self.parsed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_constant_i() {
        let lines = vec!["push constant 7".to_string()];
        let mut p = Parser::new(&lines);
        let ret = p.parse();
        assert!(ret.is_ok());
        assert_eq!(p.parsed.len(), 1);
        assert_eq!(
            p.parsed.pop(),
            Some(VmCommand::new(
                VmCmdType::Push,
                Some(Segment::Constant),
                Some(7)
            ))
        );
    }
    #[test]
    fn test_catch_invalid_cmd() {
        let lines = vec!["eat constant 7".to_string()];
        let mut p = Parser::new(&lines);
        let ret = p.parse();
        assert!(ret.is_err());
        assert_eq!(ret.unwrap_err(), "Invalid VM command type: eat".to_string());
    }

    #[test]
    fn test_catch_neg_indices() {
        let lines = vec!["push constant -2".to_string()];
        let mut p = Parser::new(&lines);
        let ret = p.parse();
        assert!(ret.is_err());
        assert_eq!(
            ret.unwrap_err(),
            "invalid digit found in string: -2".to_string()
        );
    }
    #[test]
    fn test_catch_max_num_parts() {
        let lines = vec!["push constant 7 dd".to_string()];
        let mut p = Parser::new(&lines);
        let ret = p.parse();
        assert!(ret.is_err());
        assert_eq!(
            ret.unwrap_err(),
            "Line: 'push constant 7 dd' exceeds max 3 number of parts".to_string()
        );
    }
}
