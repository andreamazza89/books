
//////////////////////////////
//////////////////////////////
//// START PUSH (CONSTANT) ///
@0
D=A

// PUSH D into the stack
@SP
A=M
M=D
@SP
M=M+1

        /////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////
        //// START POP (R13-popped value; R14-target memory address) ////
        /// 
        // pop into D and store it in R13
         
//// START POP_INTERNAL (decrements the stack pointer and stores the popped value in D)
@SP
M=M-1
A=M
D=M
//// END POP_INTERNAL

         @R13
         M=D
         
         @LCL
         D=M
         @0
         D=D+A
         @R14
         M=D 
        // set D to the popped value
         @R13
         D=M
        // set A to the target address
         @R14
         A=M
        // store the popped value into the taget address (M=D)
         M=D
        //// END POP
        
(LOOP)
//////////////////////////////
//////////////////////////////
//// START PUSH (SEGMENT) ////

         @ARG
         D=M
         @0
         D=D+A
         @R14
         M=D 
// this assumes that the previous command leaves A = R14
A=M
D=M

// PUSH D into the stack
@SP
A=M
M=D
@SP
M=M+1

//////////////////////////////
//////////////////////////////
//// START PUSH (SEGMENT) ////

         @LCL
         D=M
         @0
         D=D+A
         @R14
         M=D 
// this assumes that the previous command leaves A = R14
A=M
D=M

// PUSH D into the stack
@SP
A=M
M=D
@SP
M=M+1

//////////////////////
//////////////////////
//// START +

//// POP TWO ARGUMENTS (R13-first argument, D-second argument)

//// START POP_INTERNAL (decrements the stack pointer and stores the popped value in D)
@SP
M=M-1
A=M
D=M
//// END POP_INTERNAL

// move first argument into @R13
@R13
M=D

//// START POP_INTERNAL (decrements the stack pointer and stores the popped value in D)
@SP
M=M-1
A=M
D=M
//// END POP_INTERNAL


@R13
D=D+M

// PUSH D into the stack
@SP
A=M
M=D
@SP
M=M+1


        /////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////
        //// START POP (R13-popped value; R14-target memory address) ////
        /// 
        // pop into D and store it in R13
         
//// START POP_INTERNAL (decrements the stack pointer and stores the popped value in D)
@SP
M=M-1
A=M
D=M
//// END POP_INTERNAL

         @R13
         M=D
         
         @LCL
         D=M
         @0
         D=D+A
         @R14
         M=D 
        // set D to the popped value
         @R13
         D=M
        // set A to the target address
         @R14
         A=M
        // store the popped value into the taget address (M=D)
         M=D
        //// END POP
        
//////////////////////////////
//////////////////////////////
//// START PUSH (SEGMENT) ////

         @ARG
         D=M
         @0
         D=D+A
         @R14
         M=D 
// this assumes that the previous command leaves A = R14
A=M
D=M

// PUSH D into the stack
@SP
A=M
M=D
@SP
M=M+1

//////////////////////////////
//////////////////////////////
//// START PUSH (CONSTANT) ///
@1
D=A

// PUSH D into the stack
@SP
A=M
M=D
@SP
M=M+1

//////////////////////
//////////////////////
//// START -

//// POP TWO ARGUMENTS (R13-first argument, D-second argument)

//// START POP_INTERNAL (decrements the stack pointer and stores the popped value in D)
@SP
M=M-1
A=M
D=M
//// END POP_INTERNAL

// move first argument into @R13
@R13
M=D

//// START POP_INTERNAL (decrements the stack pointer and stores the popped value in D)
@SP
M=M-1
A=M
D=M
//// END POP_INTERNAL


@R13
D=D-M

// PUSH D into the stack
@SP
A=M
M=D
@SP
M=M+1


        /////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////
        //// START POP (R13-popped value; R14-target memory address) ////
        /// 
        // pop into D and store it in R13
         
//// START POP_INTERNAL (decrements the stack pointer and stores the popped value in D)
@SP
M=M-1
A=M
D=M
//// END POP_INTERNAL

         @R13
         M=D
         
         @ARG
         D=M
         @0
         D=D+A
         @R14
         M=D 
        // set D to the popped value
         @R13
         D=M
        // set A to the target address
         @R14
         A=M
        // store the popped value into the taget address (M=D)
         M=D
        //// END POP
        
//////////////////////////////
//////////////////////////////
//// START PUSH (SEGMENT) ////

         @ARG
         D=M
         @0
         D=D+A
         @R14
         M=D 
// this assumes that the previous command leaves A = R14
A=M
D=M

// PUSH D into the stack
@SP
A=M
M=D
@SP
M=M+1

        
//// START POP_INTERNAL (decrements the stack pointer and stores the popped value in D)
@SP
M=M-1
A=M
D=M
//// END POP_INTERNAL

        @LOOP
        D;JLT
//////////////////////////////
//////////////////////////////
//// START PUSH (SEGMENT) ////

         @LCL
         D=M
         @0
         D=D+A
         @R14
         M=D 
// this assumes that the previous command leaves A = R14
A=M
D=M

// PUSH D into the stack
@SP
A=M
M=D
@SP
M=M+1
