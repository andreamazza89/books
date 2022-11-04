# 2 Representing and manipulating information

## Information storage

As far as the system goes, the 'atom' of information is the byte (8 bits)
rather than individual bits.

Programs are presented with virtual memory and reference data at a virtual address,
which is the address of the first byte of whatever data is referenced.
In C, type information is used to determine how many bytes to read from the pointer's
virtual address. This information is lost once the program is compiled into machine-code.

(Hexadecimal representation - appreciate it's very important and understand it, but am
going to be lazy and just use an online tool or something like that whenever I need to
convert between number bases, so will skip the exercises)

### Data Sizes

Every computer has a *word size*, which indicates the nominal size of pointer data.
This size determines the maximum size of virtual address space.

Most common sizes are 32 and 64.
Programs compiled for 32-bit machines can also run on 64-bit ones, but the opposite is
not true. When we distinguish programs between 32 and 63 bits, we are referring to how
the program was compiled rather than the machine we run it on.

There are a number of data representations for a number of types, which may vary depending
on whether we are dealing with 32 or 64 bits word size. For example, an `int` takes up 4
bytes, whereas a double 8 bytes. The ISOC99 standard introduces some data types that have
the same space requirements regardless of word size.

### Addressing and byte ordering

Since program objects can span more than one byte, we need a convention for how to point
at the object, and how the object's bytes are laid out.
Pretty much everywhere the bytes are arranged contiguously and the address is pointing at
the first byte in the sequence.

There are two main ways of laying out data, one called *little endian*, the other *big endian*.
So, if we need to store the 2-byte number 1111111100000000, we have it as 11111111 _ 00000000 in
big endian (most significant bytes first) and 00000000 _ 111111111 in little endian.
These two conventions have no technological advantage over each other, so as long as one is chosen
it doesn't make any difference. This is also completely transparent to application code.
Issues can arise when binary data is sent over a network between two different machines that use
different standards.
Byte ordering becomes visible when circumventing the type system with things like type casting.

#### Problem 2.6

00000000001001111100100011111000
           *********************
  01001010000111110010001111100000

21 bits match - you can see above which ones do not match

### Representing strings

Simply an array of charcters terminated by the null (0) character.

### Representing code

Not much to say at this point other than the same high-level code can have different and incompatible
binary representations, depending on the architecture and machine, making binary code very little portable.

### Intro to boolean algebra

Invented by George Boole in the 19th century, was originally used to create an algebra for logical reasoning
(where true is 1 and false 0)

...(description of the basic operations not, and, or and xor)...

The boolean operations extend to bit vectors; the intuition is that given two bit vectors of the same size,
the result of the operation x, is given by taking the result of each pair of bits and applying the operation to
them. For example,

1111
1100 AND
.... =
1100

Boolean algebra over bit vectors gives rise to a mathematical form called *Boolean ring*.
These have interesting properties similar to those of integer arithmetic.

Bit vectors can be used to represent finite sets. Any subset of say set S can be represented
by encoding the presence or absence of a set element with either 1 or 0, but encoded backwards.
So, for example, if we have set S as {1,2,3}, then we can represent the set {2,3} with the
bit vector [1,1,0].
Once a set is encoded this way, set union and intersection are simply `|` and `&`. `~` is a set's
complement.

### Bit-level operations in C

Boolean operations can be applied to any integer type in C.

#### Problem 2.13

  a 1001
  b 1100 (expected: 0101)

  bis a b - 1101
  bis b a - 1101
  bic a b - 0001
  bic b a - 0100

  so maybe this works? bis(bic(a, b), bic(b,a))

  will try one more example just in case

  a 0111001
  b 1100101 (expecte: 1011100)

  bic a b - 0011000
  bic b a - 1000100 bis of the two: 1011100!

### Logical operations in C

These are not to be confused with bit-level operations.
Anything that's not 0 is true.

Two differences between these and the bitwise operators:

- because anything non-0 is true, 0x69 && 0x55 yields 0x01, whereas
  its bitwise counterpart would be 0x41
- short cirucit evaluation only applys to logical operations, not bitwise

### Shift operations in C

This is when we take a bit vector and shift each bit to the left or the right,
filling the remaining space with zeros.

For example, given [111], a shift of 2 to the left gives 11[100] - where
the two 11s outside on the left are only shown for illustrative purposes.
With the right shift, there is also an alternative way of doing this, which
is by padding the left hand side by repeating the most significant bit
instead of using 0s.

## Integer Representations

### Integrals

Integrals are all types defining finite integer ranges that may or may not
include negative ones.

For each type the C standards define the least range of values that must be guaranteed.

### Unsigned Encodings

B2U (binary to unsigned) is a pretty straightforward encoding, where the bitvector simply
represents a positive binary number to be converted into decimal.

An important property is that all numbers within a w-long representation has a unique
encoding as a w-bit value. This means that any decimal number has only one mapping
into a w-bit value, and viceversa.

### Two's complement

This is where the most significant bit has negative weight (i.e. tells you the sign)


