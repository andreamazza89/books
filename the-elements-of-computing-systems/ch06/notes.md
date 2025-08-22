# Assembler

We are going to write an assembler for the Hack language.
An assembler is relatively straightforward, as there is a one to one
between a symbolic instruction (i.e. M=D+M) and a binary one. The main
complicating factor is symbols, as these need to be managed in the
assembler. Symbols are simply an address that we refer to by a given
name; these have 3 forms:
- instruction address - something like (LOOP)
- predefined address - e.g. SCREEN, which is whatever address the memorymapped screen is at
- variables - places in memory that we want to call by name (e.g. 'sum')

## Handling symbols
The fact that symbolic labels (e.g. LOOP) can be used before they are defined makes
the assembler a bit trickier to write as you don't know what the label's value is until
you 'hit' it (if your program is going line by line top to bottom).
A common way to tackle this is by doing two passes through the file: a first one to
resolve symbols, the second one to generate the binary representation.
