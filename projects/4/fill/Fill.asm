// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

@i
M=0

@colour
M=0        // Default colour is white

(LOOP)
@i
D=M
@SCREEN
A=A+D      // A = (base + i)
M=-1

@i
M=M+1      // i++

@i
D=M
@8192
D=D-A
@LOOP
D;JLT

(END)
@END
0;JMP
