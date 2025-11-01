# Part I summary

Having just finished chapter 6 (Assembler), I thought I'd try write a summary of what I've learned so far.

Part one of the book builds a computer system from the ground up, all the way from hardware primitives to
where the gap between hardware and software is bridged. I really enjoyed going through it and think the
way a computer holds together is almost magical. It's also amazing how much can be done with relatively
so little.

I'll try describe the two ends of 'hardware privimitves' and a program, and then fill in the middle.

At one end, you have a system that's capable of executing an arbitrary program. The program is just 
a sequence of binary instructions, each dictating what the computer should do next. Each system will
have its own 'dictionary' of instructions, but they all have a finite amount of instructions that
can make up a program. An example of an instruction is 'load whatever number is stored at address X into register Y'
or 'add together whatever is stored in register D with whatever is stored at address E'.
These instructions are simply a sequence of 0s and 1s, and given an arbitrary program, it can be loaded into
wherever a computer expects a program to execute, and the computer will execute it.
The real neat thing is that all a program does is manipulate some area of memory which is mapped to some 'external' hardware.
For example, you could have an area of memory that maps to the pixels of a screen, and that's how you drive the screen.
It's way more complex/sophisticated in real systems, but it boils down to this.

At the other end, you have effectively two hardware primitives to pretty much build the whole system: the NAND gate
and the D flip-flop, which given a clock (I guess that's another primitive?), stores 1 bit of data.
Throught part I, we build increasingly complex components, making OR gates, AND gates, Multiplexers, all the way to
the ALU (ArithmeticLogicUnit). In order to tame complexity, we use abstraction: using primitives to build a 'bigger'
component, then giving that component a name and using it as a primitive at the next level of abstraction.
So at the end of the day we're just using NANDs and D flip-flops, but our (certainly not my) brains would not cope
with the complexity in just thinking at that level. Instead, we build a computer out of CPU, Memory and ROM.
CPU is made of an ALU, PC etc. The ALU is made of OR / Mux16, etc, OR is made of NANDs!

Motivation for having a clock and D flip-flops. With just 'logic' components, you could still do useful things, but
only one 'thing' at a time: given an ALU, you can for example add together two numbers, but that's it. If you then
want to add together two more numbers, you'd need to 'manually' update the inputs of the ALU to see the result.
So if we want some notion of 'steps' / 'before-after', we need memory. We need to 'quantise', which is to say 'slice'
time into arbitrary steps, so that the sytems knows which step we're currently at, and when the next one comes.
We are also now able to store information and move on to 'reuse' the system to compute more without losing previous
results.


Computer architecture. Mmm I'm probably missing a piece here. We have the CPU, which is responsible for fetching
instructions, executing them and writing to memory, then memory itself, which is both used by the CPU for computations
but also to interacto with the outside world, using memory-mapped IO. So for instance, the position of the mouse is
mapped to a certain 'sector' of memory, so that the CPU can read from it to figure out where the mouse currently is and
then you could have another one to 'drive' the screen. You then have to store the program instructions somewhere. This
will either be in the same memory, or in a dedicated one, each approach with its own tradeoffs. I wonder what I'm missing
if anything, I'll look it up in the book. Ok, no that's roughly it.
