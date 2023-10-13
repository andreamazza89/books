# 1 - Stop listening!

## 1.1 Project, meet complexity wall

FRP is a programming technique to improve code around *event propagation*.

## 1.2 What is functional reactive programming?

There are a number of angles/definitions, and then a stricter (though harder for me to
grasp) one.

The one I like most is: *a way of expressing event-driven logic in a modular/composable way*.

## 1.3 Where does FRP fit in? The lay of the land

FRP is the intersection between functional and reactive programming.
From the former, one big idea taken is that of compositionality.
From the latter, the idea of having a programming model that is event-based,
reacts to input and is viewed as a flow of data.

Often reactive programming applications emphasise distributed processing, which
relaxes consistency, but this is not the case in FRP, which is not particularly
suited for distributed applications.

## 1.5 State machines are hard to reason about

A state machine is a system that has a state. This is updated when an event comes in. Processing
an event might also produce some sort of output or side effect.

The book's angle is that virtually all programs can e expressed as a state machine, but these are
hard to think about and maintain, so FRP is a way to manage this complexity.

## 1.9 Why not just fix listeners?

Doing that leads to 'discovering' FRP. So, in a way FRP is just the result of
looking for a better solution than listeners.


## 1.12 How does FRP work?

Going through a small example, using 2 inputs (dates) and a button that is enabled depending on
some validations driven by these dates.

The first primitive introduces is a *Cell*, which represents a value that changes over time.

## 1.14 Thinking in terms of dependency

Traditionally, the programmer thinks of a program as a series of steps, which may or may not depend on each other.
Using event listeners and stateful objects, the programmer has to make sure the order of execution is correct; this easily turns into difficult-to-debug scenarios as
program complexity scales. With FRP, the sequencing is
maintained by the FRP library, and the developer can
just be concerned with the dependencies in the code.

Basically, with FRP, we stop having to worry about the sequencing of the program, and just think in terms of dependencies.

## 1.16 Conceptual vs. operational understainding of FRP

It's much easier and better to stick to the conceptual level (i.e. the abstractions FRP provides) rather than operation (i.e. what's really going on behind the scene).

When we run an FRP program, there are two stages. In the first on (initialisation), our FRP statements are
translated into a directed graph, which is then used
in the second stage (running) to propagate data in reaction to 'outside' events.

