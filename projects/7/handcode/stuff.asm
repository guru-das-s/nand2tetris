// Test popping logic by hand
//
// Set SP = 5
@5
D=A
@SP
M=D

// Push constant 29
@29
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Push another constant 54
@54
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Push another constant 3
@3
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Implement binary op add
//
// Get first parameter and store it in D
@SP
M=M-1
A=M
D=M
// Get second parameter
@SP
M=M-1
// Second parameter is in RAM[SP], store it in A
// after dereferencing RAM[SP] which is a pointer
A=M
A=M
// Replace RAM[SP] with result of op
D=D+A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Implement unary op not
@SP
M=M-1
A=M
D=M
M=!D
// Increment SP
@SP
M=M+1

// Pop to specific location, say, 10
@SP
// Decrement SP
M=M-1
A=M
D=M
@10
M=D
(END)
@END
0;JMP
