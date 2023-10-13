# 3 Machine-Level Representation of Programs

## 3.2 Program encodings

c source code (<>.c)
  --preprocess--> expanded c source code (with #include s and macros) (<>.c)
  --compile-----> assembly code (<>.s)
  --assemble----> object code, which is machine code without addresses of global values (<>.o)
  --link--------> merges all source code + libraries to make the executable (<>)

### 3.2.1 Machine-Level Code

There are two important abstractions:

1. The instruction set architecture (ISA) is processor-specific and describes the instructions available
  and what their effect is, as well as the processor state. Most ISAs provide the illusion that these
  instructions are executed sequentially (one after the other); this is an abstraction: processors are
  far more complex and execute instructions concurrently, but they guarantee that the result is the same
  as if they'd been processed sequentially.

2. Each program is given virtual memory, which means the program can assume all the memory available is its own.

Parts of the processor state that are not available in a C program but they are in machine code are:

- program counter (the address of the next instruction)
- register (16 locations containing 64-bit values)
- condition code registers (not 100% sure I understand this, but something like the result of a conditional op)
- vector registers (one or more int or floats)

Machine code views memory as simply a large byte-addressable array.


## 3.4 Accessing Information

An x86-64 CPU has 16 registers, each storing 64-bit, which can either be a pointer or an integer.
These can either be accessed whole (64 bits) or you can access smaller values (1, 2 or 4 bytes) for operating
on smaller integers.
Each of these registers have a different label and then 3 more labels to access the smaller 'slice' of the register.
For example, the first register can be accessed whole as %rax, or you can just get the least significant byte with %a1.

There are conventions (maybe rules too?) governing what each register is for in a typical program. For example, %rsp contains
a pointer to the end of the runtime call stack.

### 3.4.1 Operand Specifiers

Many instructions have operands (kinda like arguments), which specify for instance where to get value/s for the operation and 
where to store the result.

Operands have three types:
- immediate (constant value)
- register (reference to a register)
- memory (reference to a place in memory)

For the memory type, there are many 'addressing modes', which are elaborate ways of coming up with a memory address. The most
generic form is like 'some=fixed=offsett + register=for=base=address + register=for=index * scale=factor', where scale=factor
is either 1/2/4/8.

### 3.4.2 Data Movement Instructions

The most used type of istructions are the ones that move data from one place to the other; because of the many different
ways there are to specify an operand, there are many forms of data movement.


