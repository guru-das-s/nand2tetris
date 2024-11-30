pub const CONSTANT: &str = r#"// Push constant
@XYZ
D=A
"#;

// Only for segments Local, Argument, This, That
pub const L_A_T_T: &str = r#"// Push SEG XYZ
@SEG
D=M
@XYZ
A=D+A
D=M
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
