// Implement various arithmetic operators other than `add`
// =======================================================

// Setup
// -----

// Set SP = 5
@5
D=A
@SP
M=D

// Push constant 28 (A)
@28
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Implement neg (A)
// ---------------------

// Get first parameter and store it in D
@SP
M=M-1
A=M
D=M
// D has the parameter (A) now.
D=-D
@SP
A=M
M=D

// Increment SP
@SP
M=M+1
