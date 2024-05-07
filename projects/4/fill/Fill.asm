// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

@i         // Reset i = 0
M=0

@screeni
M=0

(LOOP)
@KBD
D=M

@colour
M=-1       // Default colour is black

@PAINT
D;JGT      // If key is pressed, colour should be black, start painting

@colour    // else, colour is white
M=0

(PAINT)
@i
D=M
@SCREEN
D=D+A      // D = (base + i)
@screeni
M=D        // screeni = (base + i)

@colour
D=M
@screeni   // Pointer, bro!
A=M
M=D        // D has to contain colour

@i
M=M+1      // i++

@i
D=M
@8192
D=D-A
@PAINT
D;JLT

@i         // Reset i = 0
M=0

@LOOP
0;JMP
