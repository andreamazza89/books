(CHECK)
// init the screen index
  @8192
  D=A
  @SCREEN
  D=D+A
  @left_to_paint
  M=D

  @KBD
  D=M

// initialise for filling if any key is pressed
  @INIT_FILL
  D;JGT

// initialise for clearning if no key is pressed
  @INIT_CLEAR
  D;JEQ

(INIT_CLEAR)
  @colour
  D=M
// nothing to do if already white
  @CHECK
  D;JEQ

// else set colour to 0 and go
  @colour
  M=0
  @PAINT
  0;JMP

(INIT_FILL)
  @colour
  D=M
// nothing to do if already white
  @CHECK
  D;JLT

// else set colour to 1 and go
  @colour
  M=-1
  @PAINT
  0;JMP

(PAINT)
  // paint the current block
  @colour
  D=M
  @left_to_paint
  A=M
  M=D

  // decrease the left_to_paint
  @left_to_paint
  M=M-1

  // stop if finished
  @left_to_paint
  D=M
  @SCREEN
  D=D-A
  @CHECK
  D;JLT

  @PAINT
  0;JMP
  
