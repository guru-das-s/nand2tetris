// Implement various arithmetic operators other than `add`
// =======================================================

// Setup
// -----

// Set SP = 5
@5
D=A
@SP
M=D

// Push constant 84 (A)
@84
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Push another constant 112 (B)
@112
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Implement and (A & B)
// ---------------------

// Get first parameter and store it in D
@SP
M=M-1
A=M
D=M
// D has second-pushed parameter (B) now. Get first-pushed parameter next
@SP
M=M-1
// Second parameter is in RAM[SP], store it in A
// after dereferencing RAM[SP] which is a pointer
A=M
// M has first-pushed parameter (A) now.

D=D&M
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
