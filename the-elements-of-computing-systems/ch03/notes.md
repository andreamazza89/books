# Sequential logic

This chapter is all about sequential logic, where the flow of time is folded in.
What we have done so far is 'combinational', meaning that the operations were not
time-dependent.

So we are going to look into all things around memory.

## Background

### The clock

This is a device that generates a constant stream of alternating 0s and 1s, fed
into all time-dependent chips for synchronisation.
The time between a 0 and a 1 is the 'cycle'.

### Flip flops

These are devices that take in the clock signal, as well as 1 bit and output what
the input was in the previous cycle. This is the 'atom' for building memory.

### Register

A register is similar to a flip flop, but instead of outputting whatever the input was
in the previous cycle, it outputs whatever the output was in the previous cycle. In other
words, it keep outputting the same value, until it is set to a new value.
This has 2 input pins: one to select whether the chip should 'update' the output, the other
is the input. So when the 'update' pin is high, the output is set to the current input, whereas
when it's low, the output simply stays the same cycle after cycle.
A multi-bit register's content is often called 'word'.

## RAM

This builds on registers to have n registers and an address input to select one for either loading
a word or outputting a word at that address.o

## Counter

This is a chip that increments its output by 1 at every cycle


In general, sequential chips use data flip-flops to introduce a time delay (the output is whatever
the input was the previous cycle) along with a feedback loop and combinational chips to go about
their jobs.

Notice that combinational chips can change their output at any point in time, whereas sequential ones
only change at the clock transitions.
This is quite handy because real-world constraints mean than whenever an operation is fed into a
combinational chip, say an ALU, reaching the final output takes _some_ time. So as long as this delay
is shorter than a clock cycle, the inconsistent state won't matter, as the values are synchronised across
the computer chips at the cycle transition.
