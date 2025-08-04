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

pub const LABEL: &str = r#"// LABEL
(XYZ)
"#;

pub const LABEL_IN_FUNC: &str = r#"// LABEL_IN_FUN_C
(FUNC$XYZ)
"#;

pub const GOTO: &str = r#"// GOTO
@XYZ
0;JMP
"#;

pub const GOTO_IN_FUNC: &str = r#"// GOTO_IN_FUN_C
@FUNC$XYZ
0;JMP
"#;

pub const IF_GOTO: &str = r#"// IF_GOTO
// first, get results of prev bool op
@SP
A=M
D=M
// pop
@SP
M=M-1
@DONTJUMP.XYZ
D;JLE
@LOOP
0;JMP
(DONTJUMP.XYZ)
"#;

pub const IF_GOTO_IN_FUNC: &str = r#"// IF_GOTO_IN_FUN_C
// first, get results of prev bool op
@SP
A=M
D=M
// pop
@SP
M=M-1
@FUNC.DONTJUMP.XYZ
D;JLE
@FUNC$LOOP
0;JMP
(FUNC.DONTJUMP.XYZ)
"#;

pub const CALL: &str = r#"// CALL
// Call: Save return address to stack
@CALLER$ret.XYZ
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
// Call: Save LCL to stack
@LCL
D=M
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
// Call: Save ARG to stack
@ARG
D=M
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
// Call: Save THIS to stack
@THIS
D=M
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
// Call: Save THAT to stack
@THAT
D=M
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
// Call: Reposition ARG = SP - 5 - nArgs
// Call: Repositioning ARG: SP already available above, save it
D=M
@5
D=D-A
@NUMARGS
D=D-A
@ARG
M=D
// Call: Set LCL to SP
@SP
D=M
@LCL
M=D
// Call: Jump to CALLEE
@CALLEE
0;JMP
(CALLER$ret.XYZ)
"#;

pub const FUNCTION: &str = r#"// FUN_CTION
(FUNC)
"#;

pub const FUNCTION_LOCAL_VAR: &str = r#"// FUN_CTION LOCAL VARIABLE INIT
@0
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
"#;

pub const RETURN: &str = r#"// RETURN
// endFrame (R13) = LCL
@LCL
D=M
@R13
M=D
// retAddr (R14) = *(endFrame - 5)
@5
A=D-A
D=M
@R14
M=D
// *ARG = pop(), well, just *(SP-1) really
// because we will be repositioning SP next
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
// SP = ARG + 1
@ARG
D=M
@SP
M=D+1
// THAT = *(endFrame - 1)
@R13
M=M-1
A=M
D=M
@THAT
M=D
// THIS = *(endFrame - 2)
@R13
M=M-1
A=M
D=M
@THIS
M=D
// ARG = *(endFrame - 3)
@R13
M=M-1
A=M
D=M
@ARG
M=D
// LCL = *(endFrame - 4)
@R13
M=M-1
A=M
D=M
@LCL
M=D
// goto retAddr
@R14
A=M
0;JMP
"#;
