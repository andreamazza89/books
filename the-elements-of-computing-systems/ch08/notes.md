# Virtual machine part 2

Along with with implementing the stack commands we've already encountered,
a Virtual Machine implementation is also responsible for the 'behind the scenes'
of running a program, including things like how to start/terminate a program and all
the overhead related to supporting functions, which involves 'suspending' the running
state of a given function while its callee is running and passing arguments between
functions.

In doing this, there are 3 challenges:
- 'sharing' the stack between functions
- managing memory (any variables that each function uses)
- return address (once the function returns, where does execution continue from?)

The first one comes for free, because function calling is 'naturally' equivalent to stack 
'movements' (last in, first out), meaning that the last function that's executing is the
one using the stack, and when it's done, its caller now (assuming it has left the stack
as it found it) has the stack back where it left it, and so on.

The memory management and return address part is a bit trickier. Each function has its own
memory segments, each with a base address stored at (ARG, LCL, THIS, THAT). So when we
need to suspend execution to move to a new function, we need to take a 'snapshot' of these
that we hold on to (push) for later, and also update them for the new function.
We call 'frame' the collection of pointers we need to pause and resume function execution.
And frames can just exist in the stack itself, which is pretty cool because the same one
thing (the stack) holds both the ongoing computation, as well as frames

