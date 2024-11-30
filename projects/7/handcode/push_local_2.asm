// =====================
// Hand-code pop local 2
// =====================

// Set LCL to 25
@25
D=A
@LCL
M=D

// Set SP = 10
@10
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

// Push local 2 begins now
// ........................
// Step 1: Get value to push
@LCL
D=M
@2
A=D+A
D=M

@SP
A=M
M=D
// Increment SP
@SP
M=M+1
