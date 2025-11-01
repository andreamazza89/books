# Virtual Machine I: Processing

The Jack language includes a virtual machine, much like Java: the high-level language
is first compiled into an intermediate representation that can be distributed to any
computer architecture, which then implements its own intermediate--binary step.

Benefits include cross-patform compatibility (your high level language only needs
compiling once into bytecode, and each platform just needs to implement the virtual
machine once) and interoperability between high-level languages that compile
to the same virtual machine.

The downside is reduced efficiency.

The program that turns bytecode into binary is called a VM translator.

In Java, a specific JVM implementation is called a JavaRuntimeEnvironment.

## Stack machine

In designing the VM language/architecture, there is a tension in how much of a gap there
is between the high level language/s on one side and the many varieties of machine languages
on the other. One way to resolve this tension is to use an abstract architecture called
stack machine. Any program, written in any high-level language can be translated into a sequence
of operations on a stack.

### Push and Pop

Nothing too exotic here.

### Stack Arithmetic

Arithmetic and logical operations can be computed via a stack machine. The way this works is
(assuming they're already on the stack) by first popping the operand/s from the stack, computing
the result, then pushing them back onto the stack.
This allows us to convert any arbitrary expression into a series of stack operations to evaluate it.
For example, (x + 3) - 1 becomes
 - push x (whatever x's value is)
 - push 3
 - add (which is pop pop, add and push the result back)
 - push 1
 - subtract (which is pop pop, subtract and push the result back)

As we'll see, the virtual machine includes a stack.

### Virtual Memory Segments

In the above, we have not defined where `x` comes from. In a high level language you can define
variables and know that they 'magically' contain/point to a value.

In the VM we are going to build (as well as in Java's JVM), there are no symbolic variables (i.e.
you can't arbitrarily define `foo` as a variable to store a value. However, there are a number of
virtual memory segments, with names like static (for static class variables), this (for instance
variables), local (for variables that are only in scope of the function) and argument (for the
arguments of the current function) and more. 

## VM Specification
There is only one data type: a signed 16-bit integer.

A VM program consists of a sequence of VM commands that are one of the following:

- push/pop commands (moving data between the stack and memory segments)
    [push | pop] segment index (where segment index is something like this 1)

- arithmetic/logical commands
    add/sub/neg/eq/gt/lt/and/or/not  (these pop as many operands as they need and then store
                                      the result back on the stack)
- branching commands

- function commands (call/return operations)

All operations take their operands from and store results to the stack.

## Implementation

Implementing the vm on a target platform (the Hack computer in or case) means figuring out
how to represent the stack and how to store memory segments.

For the stack, we just need to decide a base address for the stack (for when it's empty)
and store the current stack pointer in a register. Push and pop are implemented by writing
a value at the memory cell the pointer is pointing to and then moving the pointer along (PUSH),
or reading the value at the memory cell the pointer is pointing to and then moving it one
back. The stack operations will then all involve popping the operands, performing the operation
using the appropriate CPU instructions, then pushing the result.

For virtual memory segments, though we we are not going to do it in this chapter, we'll need to
worry about both garbage colletion (I think?) and figuring out what `local i` means at any given
time, which is hard because that depends on the current function. I am also wondering whether this
is basically the heap, and trying to figure out how it would store data that's more than 1-word
large, but it's probably easier if I pause that thought and it will become clear later.

### Standard VM mapping

Though implementors of the VM can implement it however they like (as long as the semantics are held),
it's common for the VM designers to provide some standard guidelines, for things like file names/conventions,
and memory layout.

The stack will be at addresses 256-2047, and a few of the registers have convenient aliases, like SP (for
stack pointer is the first register).

Memory management will need careful consideration (i.e. how do we make sure that memory segments don't overlap
with each other as the program runs?), but for now we just assume the 'base address' of each segment has already
been set for us and we'll return to the problem later.

