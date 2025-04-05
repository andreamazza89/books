# Machine Language

A machine language is the at the edge between software and hardware: it is made up of instructions that tell the machine
what to do, which can be boiled down to manipulating memory using a processor and a set of registers.

## Instructions

There are a ton of machine languages, as they are specialised to the various chip architecture, but they all support the following
in some shape or form:

- arithmetic and logical operation (e.g. add R1,R1,R2 -----> sum whatever is in registers 1,2 and sore the result in register 1)
- memory access; i.e. writing and reading from an address in memory
- flow control (e.g. goto)

## The Hack Machine Language

Is the one devised for this book. First, we talk about the architecture of the Hack computer, which is 16-bit von Neumann computer.

### Memory
There are two units: data memory (also called RAM) and instruction memory (ROM), each 16-bit wide and with a 15-bit address space.

The data memory is read/write, whereas the instruction memory is read only; the program is loaded in some 'exogenous' way.

### Registers
There are 3 16-bit registers:
- data D (lets you store a value)
- address A
  - you can store a value here to use here
  - or set it to the memory address you want to access, which you then do via M
  - or set the address of the next instruction, which will only be honoured if you
    deliberately add a 0;JMP for unconditional branching (there's also conditional branching)
- selected data memory M (for reading or writing from/to the address this is set to)

### Variables
Hack's assembly language also supports variables, so that you can refer to a value in your programs by name rather than by
address. The assembled machine code will use an actual address, but the assembly code will be much more readable.

### The Hack Language

All possible instructions are contained within a single 16-bit word.
There are 2 categories of instructions: A(ddress) and C(omputation) ones.
Address instructions are to set the address register to a 15-bit value, and look like 0---------- in binary and @__ in symbolic.
Computation instructions have 3 'sectors':
- the first defines what computation is to be performed (e.g. the bitwise AND of the D register and the A register)
- the second where the computation should be stored (any combination of A,D and M)
- the third is the jump directive: do we jump at all, and if we do under what condition (comparing the output of the computation to 0)

Symbols
- R1,...R15 to express intent in a program that we are manipulating RAM registers
- SCREEN, KBD - these point to the base address location of the screen and keboard memory maps
- Labels - these can be defined in a program with the (xxx) syntax to then use with GOTO instructions
- Variables - any arbitrary identifier that's not covered by the above, gets assigned a number starting from 16
              these can be used for things like storing a sum during a loop, so that we can call the address
              @sum instead of @16

