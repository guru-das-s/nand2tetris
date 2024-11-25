// Implement various arithmetic operators other than `add`
// =======================================================

// Setup
// -----

// Set SP = 5
@5
D=A
@SP
M=D

// Push constant 32767 (A)
@32767
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Push another constant 32766 (B)
@32766
D=A
@SP
A=M
M=D
// Increment SP
@SP
M=M+1

// Implement Gt
// ------------

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

// Events: push A; push B; if B < A , M[SP] = -1 else = 0
// i.e.               if (B - A) < 0, M[SP] = -1 else = 0
D=D-M
@SP
A=M
M=-1
@ISNOTGREATERTHAN
D;JLT
// We will get here only if D > 0,
// i.e. (B - A) > 0. Set M[SP] = 0
@SP
A=M
M=!M
(ISNOTGREATERTHAN)
