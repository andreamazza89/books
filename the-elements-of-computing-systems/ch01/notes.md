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


