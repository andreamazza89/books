# Boolean Arithmetic

In this chapter we will build an Arithmetic Logic Unit built out of the chips made in the previous chapter.

## Operations

all operations can be built off of addition

## Addition

Two binary numbers can be added bitwise right to left, same algorithm as the one learned in elementary school.

## Signed binary numbers

Talks about two's complement, which has a number of great properties, including:

- same hardware can be used to represent/manipulate signed and unsigned integers
- negating a number is a simple operation
- subtraction is done with addition

## Specification

### Adders

- half adder (adding two bits)
result is a two-bit number, where one is called `sum`, the other `carry`

- full adder (adding three bits)
same as above, but adding together 3 bits

// the two above take their name from the fact that you can use 2 half adders to make a full adder
- adder (adding two n-bit numbers)

- incrementer (adds one to a number)
especially useful to figure out the next instruction address from memory

### ALU

Whereas all the chips so far are completely generic/system-agnostic, this ALU is specific to Hack, the
computer we build in this course. This still reflects the design principles applicable to ALUs though.

A couple of decisions made by Hack with regards to the ALU:

- only performs integer arithmetic
- supports 18 arithmetic-logical functions

