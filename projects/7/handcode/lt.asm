// Implement various arithmetic operators other than `add`
// =======================================================

// Setup
// -----

// Set SP = 5
@5
D=A
@SP
M=D

// Push constant 891
@891
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Push another constant 892
@892
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Implement Lt
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

// Events: push A; push B; if B > A , M[SP] = -1 else = 0
// i.e.               if (B - A) > 0, M[SP] = -1 else = 0
D=D-M
@SP
A=M
M=-1
@ISNOTLESSTHAN
D;JGT
// We will get here only if D < 0,
// i.e. (B - A) < 0. Set M[SP] = 0
@SP
A=M
M=!M
(ISNOTLESSTHAN)
