// Implement various arithmetic operators other than `add`
// =======================================================

// Setup
// -----

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

// Push another constant 29
@29
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Implement Eq
// ------------

// Get first parameter and store it in D
@SP
M=M-1
A=M
D=M
// D has first parameter now. Get second parameter next
@SP
M=M-1
// Second parameter is in RAM[SP], store it in A
// after dereferencing RAM[SP] which is a pointer
A=M
// M has second parameter now.
D=D-M
@SP
A=M
M=-1
@ISEQUAL
D;JEQ
// We will get here only if
// D is not zero, i.e. the two numbers
// are dissimilar.
@SP
A=M
M=!M
(ISEQUAL)
