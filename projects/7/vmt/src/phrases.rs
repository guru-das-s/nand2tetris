pub const PUSH_CONSTANT: &str = r#"@XYZ
// Push
D=A
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
A=M
D=D+A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
"#;
