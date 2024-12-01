pub const CONSTANT: &str = r#"// Push constant
@XYZ
D=A
"#;

pub const POINTER: &str = r#"// Push pointer XYZ
@XYZ
A=M
D=A
"#;

pub const POINTER_ADDRESS: &str = r#"// Compute addr of pointer XYZ
@XYZ
D=A
"#;

// Only for segments Local, Argument, This, That
pub const SEGMENT: &str = r#"// Push SEG XYZ
@SEG
D=M
@XYZ
A=D+A
D=M
"#;

pub const SEGMENT_ADDRESS: &str = r#"// Compute addr of SEG XYZ
@SEG
D=M
@XYZ
D=D+A
"#;

pub const TEMP: &str = r#"// Push temp XYZ
@5
D=A
@XYZ
A=D+A
D=M
"#;

pub const TEMP_ADDRESS: &str = r#"// Compute addr of temp XYZ
@5
D=A
@XYZ
D=D+A
"#;

// XYZ will be handled by VmCommand::code_segment_i(), and
// FILE will be handled by Asmwriter::write().
pub const STATIC: &str = r#"// Push Static XYZ
@FILE.XYZ
A=M
D=A
"#;

pub const STATIC_ADDRESS: &str = r#"// Compute addr of Static XYZ
@FILE.XYZ
D=A
"#;

pub const PUSH: &str = r#"// Push
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
"#;

pub const ADD: &str = r#"// ADD
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D+M
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
"#;

pub const EQ: &str = r#"// EQ
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@SP
A=M
M=-1
@ISEQUAL.XYZ
D;JEQ
@SP
A=M
M=!M
(ISEQUAL.XYZ)
// Increment SP
@SP
M=M+1
"#;

pub const LT: &str = r#"// LT
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@SP
A=M
M=-1
@ISNOTLESSTHAN.XYZ
D;JGT
@SP
A=M
M=!M
(ISNOTLESSTHAN.XYZ)
// Increment SP
@SP
M=M+1
"#;

pub const GT: &str = r#"// GT
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@SP
A=M
M=-1
@ISNOTGREATERTHAN.XYZ
D;JLT
@SP
A=M
M=!M
(ISNOTGREATERTHAN.XYZ)
// Increment SP
@SP
M=M+1
"#;

pub const SUB: &str = r#"// SUB
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
"#;

pub const NEG: &str = r#"// NEG
@SP
M=M-1
A=M
D=M
D=-D
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
"#;

pub const AND: &str = r#"// AND
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D&M
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
"#;

pub const OR: &str = r#"// OR
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D|M
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
"#;

pub const NOT: &str = r#"// NOT
@SP
M=M-1
A=M
D=M
D=!D
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
"#;

pub const POP_PRE: &str = r#"// POP prologue
@SP
M=M-1
A=M
D=M
@SP
M=M+1
A=M
M=D
@SP
M=M-1
"#;

pub const POP: &str = r#"// POP
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
@SP
A=M
D=M
@SP
M=M-1
A=M
A=M
M=D
"#;
