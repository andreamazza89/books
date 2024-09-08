# (Intro)

We are going togo well beneath the surface, and understand how computers work from their lowest-level components
all the way up to something as trivial as a Hello World program.

Which computer shall we build? It turns out all general-purpose computers are 'the same' in that they all are made
up of Nand gates and let people write any software they can think of.

The book then describes abstraction Vs implementation and how every layer of the computer architecture relies on just
the abstraction of a lower one, and that's how we can deal with enormous complexity.

# Boolean Logic

All digital devices have elementary logic gates at their core, which can be made in a number of ways, but all oblige 
to the same abstraction, which is to manipulate input signal/s into output signal/s.

We'll be taking the Nand gate, and we'll use it to create many other kinds of gates.

?~ I wonder what makes the Nand gate so 'atomic', as in, is it the only one that can be used to derive the others? Or
can they all be derived from each other? If the former, what's so special about Nand?

## Boolean algebra

Some subsets of logical operators can be used to express all other operators, with {AND,OR,NOT} being one of them.
{NAND} is another one.

From any given boolean expression, we can derive its truth table. The opposite doesn't seem obvious, but-
Theorem: from any given truth table, we can always systhesise a Boolean expression.

It's useful to be able to move between these two representations because say we have a truth table of what we want a
system to achieve, the expression conversion will tell us what components to use.

Also notice that whereas a truth table is unique, any given boolean expression has several (probably infinite) equivalents.
Simplifying boolean expressions is how we make them easier to grok and optimize hardware.

## Logic gates

These are the 'embodiment' of boolean operators in the real world, where binary inputs are turned into binary output 
according to a truth table.
Gates can be composed to form 'composite' gates or arbitrarily complex logic.

## Hardware Contruction

Thought experiment to build an Xor gate out of and,not,or gates in the garage.

## Hardware Description Language

HDL is a formalism to describe the makeup of a chip with a language. Once the designer has made one, it can be fed
to software that generates a simulation of the chip described and can run inputs on it to check that it matches the
expected outputs and measure the chip's power/performance characteristics.

A HDL program has two sections: header and parts. The former describes the overall interface of the chip it describes. 
The latter is a list of all the gates used and how they are interconnected.

You can then write a test script, which simulates whatever input you provide and generates a file with the outputs
given by the system designed.

# Specification

They introduce the Nand gate and Basic Logic Gates (not, or, xor, multiplexer and demultiplexer)

A multiplexer takes three inputs, 2 are data sources, the other selects with one should be 'read' from in the output. A
better name for it would be 'selector'.

A demultiplexer does the opposite of the above: given one 'data' input, it routes it to two different outputs, depending on
the 'selecting' input.

## Multi-Bit versions of Basic Gates
These are basic gates that can process multiple bits at one time rather than just one. An example is the bitwise And for
two 16-bit inputs.

Notice that HDL programs treat multi-bit values like single-bit ones, with the excpetion that the values can be indexed to access
the individual bits. Bits are indexed right to left, with the rightmost being the 0th.

## Multi-Way Versions of Basic Gates

These are just an 'extended' version of basic gates. For example, a multi-way or might have 8 inputs and the output will be 1 if any
of the inputs are 1.

A multi-way/multi-bit multiplexer combines multi-bit and multi-way together, for example, a 4-way multiplexer uses two inputs to select
one of 4 multi-bit inputs to send to the output.

A demultiplexer version of the above also exists: there is one n-bit input, and m n-bit outpus, with enough selection bits to cover all outputs.

# Implementation

Now a discussion on how the above are implemented.

## Behavioural simulation

This is to experiment with chip interfaces before actually have to implement their hardware description. To do this, we simply 
implement the chip's desired behaviour in code.

Now onto the project - implementing the chips described in this chapter

--
(working on Or from not/and)

not(and(x,y))                                x
not(and(not(x), y))                          x
and(not(x), y)                               x
   1 0 | 0
   1 1 | 0
   0 1 | 1
   0 0 | 0
not(and(not(x), not(y)))
   1 0 | 1 
   1 1 | 1
   0 1 | 1
   0 0 | 0

--
(working on XOr from not/and/or)

or(not(and(x,y), or(x,y)) 
   1 0 | 1
   1 1 | 1 x
   0 1 | 
   0 0 | 

and(or(x,y), not(and(x,y))
   1 0 | 1
   1 1 | 0
   0 1 | 1
   0 0 | 0


--
(working on Multiplexer from Nand, And, Not, Or, Xor)

x y s
0 0 0 | 0
0 1 0 | 0
1 0 0 | 1
1 1 0 | 1
1 1 1 | 1
0 0 1 | 0
0 1 1 | 1
1 0 1 | 0


and(or(s,x), not(and(s,y)) )
x y s      nope
0 0 0 | 
0 1 0 | 
1 0 0 | 
1 1 0 | 
1 1 1 | 
0 0 1 | 
0 1 1 | 
1 0 1 | 


or(or(not(s), x), and(s, y))
x y s
0 0 0 | nope
0 1 0 | 
1 0 0 | 
1 1 0 | 
1 1 1 | 
0 0 1 | 
0 1 1 | 
1 0 1 | 1


or(and(not(s),x), and(s, y)) !
x y s
0 0 0 | 0
0 1 0 | 0
1 0 0 | 1
1 1 0 | 1
1 1 1 | 1
0 0 1 | 0
0 1 1 | 1
1 0 1 | 0 

--
(working on DeMultiplexer from Nand, And, Not, Or, Xor, Multiplexer)

we're aiming for

in s   out1  out2
0  0 | 0     0
0  1 | 0     0
1  0 | 1     0
1  1 | 0     1

out2 -> and(in, s)
out1 -> and(in, not(s))

.. continuing the future ones directly on the hdl file

