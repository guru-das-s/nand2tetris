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
// Step 1: Get data to pop
@SP
M=M-1
A=M
D=M
// Step 2: Push this data to stack
@SP
M=M+1
A=M
M=D
// Now last two elements in stack
// are the same, with SP pointing
// to the last NON-EMPTY element.
// Set SP to the second-to-last spot.
@SP
M=M-1
// Step 3: Calculate address to pop to
@LCL
D=M
@2
D=D+A
// Set SP to this value
@SP
A=M
M=D
// Increment SP
@SP
M=M+1
// Now last two elements of stack are
// (N-1) ----- address
// (N)   ----- data     <SP points here>
// Now, finish it off
@SP
A=M
D=M
@SP
M=M-1
A=M
A=M
M=D
// Now SP points to right place.
