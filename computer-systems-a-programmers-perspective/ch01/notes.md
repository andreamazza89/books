# Chapter 1 - A Tour of Computer Systems

## 1.1 Information Is Bits + Context

Any information (text files, images, programs, memory) is exists as binary digits. So context is
essential to be able to interpret these bits and be able to distinguish between different types
of data, but also within the same kind. For example, text can be stored using different
encodings (e.g. ASCII UTF-8).

## 1.2 Programs are translated by other progrms into different forms

Similar to how everything is eventually represented as a sequence of bits, any program must
eventually be converted into low-level machine language in order to be run on a machine.

So the high level programs we write are translated into *executable object files* by other
programs, with steps that include compilation and linking. All the steps are orchestrated
by a *compiler driver*. In c, `gcc` is a compiler driver.


## 1.3 It pays to understand how compilation systems work

- optimizing program performance
- understanding weird link-time errors
- avoiding security holes

## 1.4 Processors read and interpret instructions stored in memory

A tour of the most important elements of computer hardware:

- buses       | transport bits between all the components
- I/O devices | connections to the outside world, like a keyboard, monitor or disk
- main memory | temporary storage holding program instructions and the data it manipulates
- processor   | executes instructions that are stored in main memory. The program counter keeps
                track of the address where the current instruction is in main memory.
                A processor's cycle is basically to read the current instruction, execute it and
                move on to the next instruction (which may not be contiguous if the current
                instruction was a *jump*). There are only a small set of opterations, like load
                store and jump. We distinguish between a processor's *architecture* (the abstraction
                that keeps things simple and include the few simple operations described above) and
                *microarchitecture* (the much more complicated system that honours the abstraction
                but runs much faster).

With the above in mind, here's what (roughly) happens when we type `./hello` in the shell:

1. The characters `. / h e ...` are read from keyboard, loaded into the registers and then into main memory.
2. The `hello` executable is loaded from disk directly into memory, bypassing the processor.
3. The `main` routine is executed, which eventually loads the text `hello, world!` into registers and prints it
   out to the screen.

## 1.5 Caches matter

In running programs, a lot of information moves from A to B. For example, from Disk to memory, or from memory
to registers, so it's important that we move information as fast as possible.

Something to bear in mind is that different kinds/sizes of memory have different speeds; for example, on a typical
system the disk might be 1000 times larger than memory, but it would take the processor 10 million times longer to read
a word from disk compared to memory!

Another real-world constraint is the `processor-memory gap`: it is easier and cheaper to make processors run faster than it
is to make memory run faster.
To overcome this issue, system designs include smaller, faster storage devices called `cache memories` (or `caches`), which
serve as a temporary form of storage that the processor is likely to need in the near future.
Caching exploits *locality*, which is the fact that programs tend to access data/code in localised regions.

## 1.6 Storage Devices for a Hierarchy

Nothing surprising here: we go from fastest/smallest (registers on a processor) to slowest/biggest (network-attached storage).
Each level can be seen as a cache for the level below.

## 1.7 The Operating System Manages the Hardware

The operating system is an abstraction layer between the application programs and the actual hardware.
Applications cannot manipulate hardware directly, but must go through the operating system. This is to abstract away
the complexity and variety of hardware you might run a program on and also to stop applications from misuse hardware.

There are 3 main forms of abstraction provided by the operating system:

### Processes

A process is an abstraction around a single program execution. It provides the illusion that the program has exclusive
access to the processor, memory etc. However, in order to run multiple programs at the same time, the operating system
is simply interleaving execution of different processes to time-share the same processor. It does so using *context
switching*, whereby a program's context (e.g. the current value on the ProgramCounter) is stored away and restored to
swtich between different programs running on the same CPU.

The subset of the operating system managing context switching is called the *kernel*, which is always present in memory
and is not a process - it's just always there.
A *system call* is when a process needs some action from the operating system and transfers control to the kernel.

### Threads

A process is made up of multiple execution units, called *threads*, which are all running in the same process' context and
share the same code/data.

### Virtual Memory

This is the abstraction whereby a process has the illusion of having exclusive use of the main memory.
Each process views memory as *virtual address space*, which has well-defined regions, starting from the lowest address:

- Program code and data | this is where a program is loaded from disk, followed by global C variables.
                          this area is fixed in size once the program begins.
- Heap                  | this is for run-time data and is not fixed in size, expanding and contracting as the program runs.
- Shared libraries      | code + data for shared libraries, such as the C standard libary and the Math library.
- Stack                 | another dynamic area, expands and contracts as the program run for each function call.
- Kernel virtual memory | read-only area that is used when a program transfers control to the kernel.

### Files

A file is a sequence of bytes. This is the model for every I/O device, including disks, keyboards, etc.
This is a powerful and elegant abstraction: applications can be blissfully unaware of the underlying differences
between storage/transmission mechanisms.

## 1.8 Systems communicate with other systems using networks

From a system's point of view, the network is just another I/O device.

## 1.9 Important themes

### Amdahl's Law

The law states that when a system's component is improved, the overall improvement depends on both how much
faster the component now is, but also how much it contributed to the final task.
This feels intuitive: if a system component is but a small fraction of all the system's work, even making it a ton
faster will have a modest effect overall. Conversely, if you pick a component that does a large proportion of the work,
even a small improvement will have significant results overall.

Problem 1.1

- We just need to substitue the values into the speedup formula.
  The improvemet is by a factor of 1.5x, whereas the fraction of the total journey is 0.6 (60%).
  Giving 1 / (1-0.6) + (0.6 / 1.5) = 1.25, which means it will take  25/1,25 = 20 hours.

- 1 / 0.4 + (0.6 / X) = 1.67
  1 = 1.67 * (0.4 + (0.6 / X))
  X = 1.67 * ((0.4 + (0.6 / X)) * X)
  X = 1.67 * (0.4 X + 0.6)
  X = 0.668 X + 1.002
  X - 0.668 X =  1.002
  0.332 X =  1.002
  X = 3.01807

Problem 1.2

- 1 / (1-0.9) + (0.9 / X) = 4
  1 = 4 * (0.1 + (0.9 / X))
  X = 0.4 X + 3.6
  X - 0.4 X = 3.6
  0.6 X = 3.6
  X = 6

### Concurrency and Parallelism

*Concurrency*: the ability of a system to do multiple things at once
*Parallelism*: doing the same thing in parallel to do it faster

---
Thread-level concurrency. This is whn you can have multiple control flows within the same process.
At the beginning, the concurrent aspect was only simulated on a single core, which allowed for time
sharing the same machine amongs multiple users. When running on a single processor, this is called
*uniprocessor system*, as opposed to a *multiprocessor system*.

*multi-core* and *hyper-threaded* are different types of multiprocessor systems.
The former features multiple cores ona single chip, each core with its own low-level cache.
The latter has some components doubled up in the same core, which allows it to context switch faster
than its counterpart. This is turn means faster overall execution, as it can decide to switch between
threads while loading data for example.

---
Instruction-level parallelism. Processor cores do not necessarily execute one instruction per clock cycle.
This kind of parallelism refers to when a processor is capable of executing more intructions at one time
and as a result multiple instructions per clock cycle.
Processors that can execute more than one instruction per cycle are known as *superscalar*.

---
Single instruction, multiple data parallelism. This is when the processor defines an instruction that results
in multiple operations. For example, adding 8 pairs of numbers together. This kind of parallelism is especially
usefuul in image, sound and video data processing.
To best leverage this kind of parallelism, you should use the `vector` data type.


