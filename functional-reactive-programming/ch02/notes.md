# 2 - Core FRP

## 2.1 The Stream type: a stream of events

## 2.3 The components of an FRP system

An FRP system is usually a lightweight library consisting of two classes (Stream / Cell) and a bunch of
primitives that operate on them. There are also *operations* on streams/cells, which are made up of
primitives and simply take streams/cells and spit out streams/cells.

FRP at its core is pretty much combining primitives/operations applied to streams/cells.

We are guaranteed that no matter how complex the transformations are, the resulting streams/cells have
predictable (?) behaviour (? maybe properties?). This is given from the property of *compositionality*,
whereby an expression's meaning is simply given by the smaller expressions that make it up and the
rules used to combine them.

## 2.5 The Cell type: a value that changes over time

In FRP Cells model state (a value that changes over time), whereas streams model state changes.

### 2.6.1 Simultaneous events

FRP libraries handle the processing of new events (or changes to the value of a cell) within a transaction,
in a similar fashion to database transactions.

So, when two events exist within the same transaction, they are simulataneous, regardless of small differences in
timing when they were fired.

When you merge two streams and two events are simultaneous, there needs to be a strategy for merging.
In sodium, the *merge* primitive also requires a *combine* function that takes the two simultaneous events and
makes them into one - this is used whenever two events occur simultaneously.

The fact that two simultaneous events are collapsed when streams are merged means that we are guaranteed that inside
one stream there is only one event per transaction and within a transaction all events occur at the same time
(i.e. there's no need to worry about ordering).

Notice that the above is library-dependent: not all libraries enforce one event per stream per transaction.

### 2.9.3 Accumulator code

The example shows how you can combine together *snapshot* and *hold* to accumulate values (a-la reduce).

This is done by definining like so: say we have a cell 'acc' with the latest accumulated value; this cell
is defined as the *hold* of an 'update' stream. This stream is defined as the *snapshot* of the 'acc' cell itstelf.
Basically this is building a reduce using FRP, where the cell 'acc' is the accumulated output and *snapshot* is the
accumulating function, using the latest value from the cell and the stream value that was last fired.
(the above explanation probably makes very little sense if I read it again in an hour! - but that's ok).


