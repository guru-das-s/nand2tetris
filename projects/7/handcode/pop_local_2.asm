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

// Pop local 2 begins now
// ........................
// Step 1: Get value to pop
@SP
M=M-1
A=M
D=M
// Store it in first scratch register
// Only R13, R14 and R15 are free to use
// in my view, the TEMP segment taking up
// R5 - R12 for itself.
// The BasicTestVME.tst does not seem to be
// using R13 - R15 in my test run, or it
// must be cleverly clearing them to not
// give away this insight.
@R13
M=D
// Now R13 has the value! Remember this.

@LCL
D=M
@2
D=D+A
// Now D has mem location LCL+2
// Store it in second scratch register
@R14
M=D
// Now R14 has the target mem location. Remember this too.

// Now, triumphantly do *(M[R14]) = M[R13]
// (R14 is a pointer)
@R13
D=M
@R14
A=M
M=D
