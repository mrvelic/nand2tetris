// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

(DISPRST)
    // set sn to the beginning of the screen
    @SCREEN
    D=A
    @sn
    M=D

(LOOP)
    // if sn == KBD (end of display), reset sn loop counter
    @sn
    D=M
    @KBD
    D=D-A
    @DISPRST
    D;JEQ

    // read the KBD value
    @KBD
    D=M
    
    // skip setting the -1 if the KBD is 0
    @SETDISP
    D;JEQ
    D=-1

(SETDISP)
    // set the display value (0 if no key pressed, -1 if key pressed)
    @sn
    A=M
    M=D

    // sn = sn + 1
    @sn
    M=M+1
    @LOOP
    0;JMP


