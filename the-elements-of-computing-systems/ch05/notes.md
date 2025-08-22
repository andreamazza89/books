# Computer Architecture

## Fundamentals

### Stored program
The stored program concept is the breakthrough invention of making a program's instructions data.
This means that although a computer is a finite device made of finite components, you have a
virtually infinite set of programs that you can write and run on it.

### Von Neumann Architecture
It's a moel for how to build a general purpose computer.
It has 3 main components: I/O, CPU and memory. The program to be run is stored in memory.

### Memory
This is a linear sequence of registers, which can be _addressed_ to write a value to or retrieve
the current value. They all have the same width (i.e. length in bits).

We distinguish between data and instruction memory. The former can be used by a running program,
whereas the latter is where the program's instructions are stored and run from. One could either
have both kinds in the same address space, or have physically separate memories for the two.
Each option has its tradeoffs (discussed later in the book)

### CPU
Loads an instruction and executes it. Each intruction tells it what computation to perform, what
registers to access/modify, and where to 'go' for tthe next instruction. To do all this, it uses
an ALU, registers and a control unit.
The registers are pretty much equivalent to memory, so why not just use data memory? This is because
of the delay in accessing memory versus registers, as the latter are co-located in the CPU. These
registers are used for a number of purposes: storing temporary values, for addressing memory, program
counter (keeping track of the next instruction address) and storing the current instruction.
The control unit takes a single instruction and breaks it down into the actual 'micro-codes' - these being
the physical inputs to the ALU, registers and memory.

## The Hack Hardware platform
Is made of a CPU, 3 registers, data/instruction memory and 2 I/Os - keyboard and screen.

### CPU
If it gets an A-instruction, then it simply commits the value to the A register.
For C-instructions, it uses the ALU to compute the output and then store it in whatever
destination/s the instruction wants it in; if that includes the M destination, then the 
output of the CPU is set to the ALU's output, and there is a 'writeM' bit that will be set
to 1 (meaning do write the ALU's output to the address A is pointing at)

## Instruction memory
This is simply a 32K ROM, with the input coming from the CPU's pc register (selecting the next
instruction) and the output going into the CPU's instruction input (feeding the current instruction)

## Perspective
Even though the computer is but primitive/minimal version of real computers,
this is mainly in quantitative terms (i.e. number of registers, memory, etc),
but not qualitative: this is a general purpose computer, following the von
neumann architecture.

There two broad categories of computers: general purpose and single-purpose.
The former is for computers that are expcted to run several and varied
programs as you'd expect on a personal computer, whereas the latter is for
very specific functions, like chips for automotive, industrial controls,
video processing, etc.

There is an interesting design decision around storage for instructions vs
data: do these two share the same address space, or do they have different
spaces?

If they don't share address space (also known as the Harvard architecture)
then you have to have separate memory chips (maybe a downside as there's
extra cost?), but the advantage is that you can fetch the instruction and
manipulate memory at the same time (in one cycle), whereas with a single
address space you first need to fetch the instruction, store it somewhere,
and then can you operate on data memory.

