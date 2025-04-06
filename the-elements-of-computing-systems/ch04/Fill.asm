(CHECK)
  @KBD
  D=M
  @INIT_FILL
  D;JGT
  @INIT_CLEAR
  D;JEQ
  @CHECK
  0;JMP

// TODO - just do the same looping, but use a variable for what we want all pixel blocks to be

(INIT_CLEAR)
  @8192
  D=A
  @left_to_clear
  M=D
  @CLEAR
  0;JMP

(CLEAR)
  // whiten
  @left_to_clear
  D=M
  @SCREEN
  D=D+A
  A=D
  M=0

  // decrease the left_to_clear
  @left_to_clear
  M=M-1

  // stop if finished
  @left_to_clear
  D=M
  @CHECK
  D;JLT

  @CLEAR
  0;JMP
  

(INIT_FILL)
  @8192
  D=A
  @left_to_fill
  M=D
  @FILL
  0;JMP

(FILL)
  // blacken the block
  @left_to_fill
  D=M
  @SCREEN
  D=D+A
  A=D
  M=-1

  // decrease the left_to_fill
  @left_to_fill
  M=M-1

  // stop if finished
  @left_to_fill
  D=M
  @CHECK
  D;JLT

  @FILL
  0;JMP
  
