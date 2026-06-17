// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
// The algorithm is based on repetitive addition.

    // clear memory
    @n
    M=0
    @R2
    M=0

(LOOP)
    // if n == R1 goto END
    @n
    D=M
    @R1
    D=D-M
    @END
    D;JEQ

    // R2 = R2 + R0
    @R0
    D=M
    @R2
    M=D+M

    // n = n + 1
    @n
    M=M+1
    @LOOP
    0;JMP

(END)
    @END
    0;JMP
