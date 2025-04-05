// set the output to 0
  @2
  M=0

(LOOP)
  // load second num
  @1
  D=M
  // if it's zero, go to end
  @END
  D;JEQ
  // else add the first number to the output
  @0
  D=M
  @2
  M=D+M
  // and decrease the second number
  @1
  M=M-1
  // repeat
  @LOOP
  0;JMP

(END)
  @END
  0;JMP
