use crate::spec::{Comp, Destination, Jump};

#[derive(Debug)]
pub enum HackLine {
    Whitespace,
    Comment,
    A {
        value: u16,
    },
    C {
        dest: Option<Destination>,
        comp: Option<Comp>, // If None, is error.
        jump: Option<Jump>,
    },
}

impl HackLine {
    fn is_whitespace(line: &str) -> bool {
        line.chars().all(char::is_whitespace)
    }

    fn is_comment(line: &str) -> bool {
        line.starts_with("//")
    }

    fn is_a_type_instruction(line: &str) -> Option<HackLine> {
        if line.starts_with("@") {
            let val = line[1..].trim().parse().ok()?;
            Some(HackLine::A { value: val })
        } else {
            None
        }
    }

    fn parse_dest(dest: &str) -> Option<Destination> {
        match dest {
            "" => Some(Destination::Null),
            "M" => Some(Destination::M),
            "D" => Some(Destination::D),
            "DM" | "MD" => Some(Destination::DM),
            "A" => Some(Destination::A),
            "AM" => Some(Destination::AM),
            "AD" => Some(Destination::AD),
            "ADM" | "AMD" => Some(Destination::ADM),
            _ => None,
        }
    }

    fn parse_comp(comp: &str) -> Option<Comp> {
        match comp {
            "0" => Some(Comp::Zero),
            "1" => Some(Comp::One),
            "-1" => Some(Comp::MinusOne),
            "D" => Some(Comp::D),
            "A" => Some(Comp::A),
            "M" => Some(Comp::M),
            "!D" => Some(Comp::NotD),
            "!A" => Some(Comp::NotA),
            "!M" => Some(Comp::NotM),
            "-D" => Some(Comp::MinusD),
            "-A" => Some(Comp::MinusA),
            "-M" => Some(Comp::MinusM),
            "D+1" => Some(Comp::DplusOne),
            "A+1" => Some(Comp::AplusOne),
            "M+1" => Some(Comp::MplusOne),
            "D-1" => Some(Comp::DminusOne),
            "A-1" => Some(Comp::AminusOne),
            "M-1" => Some(Comp::MminusOne),
            "D+A" => Some(Comp::DplusA),
            "D+M" => Some(Comp::DplusM),
            "D-A" => Some(Comp::DminusA),
            "D-M" => Some(Comp::DminusM),
            "A-D" => Some(Comp::AminusD),
            "M-D" => Some(Comp::MminusD),
            "D&A" => Some(Comp::DandA),
            "D&M" => Some(Comp::DandM),
            "D|A" => Some(Comp::DorA),
            "D|M" => Some(Comp::DorM),
            "" | _ => None,
        }
    }

    fn parse_jump(jump: &str) -> Option<Jump> {
        match jump {
            "" => Some(Jump::Null),
            "JGT" => Some(Jump::JGT),
            "JEQ" => Some(Jump::JEQ),
            "JGE" => Some(Jump::JGE),
            "JLT" => Some(Jump::JLT),
            "JNE" => Some(Jump::JNE),
            "JLE" => Some(Jump::JLE),
            "JMP" => Some(Jump::JMP),
            _ => None,
        }
    }

    fn is_c_type_instruction(line: &str) -> Option<HackLine> {
        // C-instruction: dest = comp;jump
        // Either the dest or jump fields may be empty.
        // If dest is empty, the "=" is omitted;
        // if jump is empty, the ";" is omitted.

        let num_equals: usize = line.matches("=").count();
        let num_semicolons: usize = line.matches(";").count();

        if num_equals > 1 || num_semicolons > 1 {
            return None;
        }

        // Split at "=" first
        let (dest, rest) = line.trim().split_once("=").unwrap_or(("", line.trim()));
        // Split at ";" next
        let (comp, jump) = rest.split_once(";").unwrap_or((rest.trim(), ""));

        let c = HackLine::C {
            dest: HackLine::parse_dest(dest),
            comp: HackLine::parse_comp(comp),
            jump: HackLine::parse_jump(jump),
        };

        if matches!(
            c,
            HackLine::C { comp: None, .. }
                | HackLine::C { dest: None, .. }
                | HackLine::C { jump: None, .. }
        ) {
            // None of these fields can be None.
            // None means that an invalid value has been encountered.
            // The Destination and Jump fields can be empty = valid Null
            // There is no valid value for a "Null" comp value, so
            // the invalid comp value would have to be None. Which is not
            // allowed.
            return None;
        }

        Some(c)
    }

    pub fn parse_line(line: &str) -> Result<Self, String> {
        if HackLine::is_whitespace(line) {
            Ok(HackLine::Whitespace)
        } else if HackLine::is_comment(line) {
            Ok(HackLine::Comment)
        } else if let Some(a_instr) = HackLine::is_a_type_instruction(line) {
            Ok(a_instr)
        } else if let Some(c_instr) = HackLine::is_c_type_instruction(line) {
            Ok(c_instr)
        } else {
            Err(format!("Line '{}' does not adhere to Hack ASM spec", line))
        }
    }
}
