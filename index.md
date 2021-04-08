# Fumola: A Programming Language Project

## Technical summary

Fumola programs consist of **networks of processes** that operate over a **shared symbolic space, in shared symbolic time**.

The collective behavior of these processes on shared space-time consists of traced symbolic storage operations that permit them to communicate, to co-operate,
and to reflect collectively on their shared history of trace-store behavior.

The trace-store consists of a history of the following per-process operations:

#### Put operation

A **put** operation has the effect of storing a new value and addressing it for subsequent **link** and **get** operations.

#### Get operation

A **get** operation retrieves the current stored value of a symbolic address.

A **get** operation uses the symbolic address produced by a **put** for the value.

#### Link operation

A **link** operation uses a symbolic query to locate and secure the address of a stored value for subsequent **get** operations.

Each **link** operation blocks the linking process until the symbol is available.

By contrast, by virtue of being type-correct **get** operations always succeed immediately, as do **put** operations.

#### Split operation

Finally, processes may split themselves, subdividing their process control structure along a symbolic name, and creating "forked" child processes using that new name.

### Network state as its trace history

A Fumola process network traces itself.

The network globally records each individual operation of each process (**put, link, get, split**),
using this history as its state for defining the meaning of subsequent operations it appends by running further.

Incremental computing techniques compress and reuse trace history.  The structure of symbolic names guides the (Nominal Adapton-based) reuse algorithm.  (To do.)

Administrative operations (TBD / To do) dispose of permanently disused history.

### Security through a type system with symbolic verification

To avoid miscommunication, mistakes and privacy concerns, a Fumola
process network cooperates using a form protocol verification.

Fumola networks encode their protocols into a special [program
logic](https://en.wikipedia.org/wiki/Hoare_logic) integrated into the
Fumola type system.

This type system includes (abstractions of) the symbolic process
effects **put**, **link**, **get** and **split**.

Within this system, processes model their behavior statically using a
capabilities-based security model based on a formal logic of Fumola
network process state.

The symbolic data of the past within the network becomes input for
future process behavior (via link and get operations), which creates
(yet) more symbolic trace data to process in the future.


## Big picture motivations

Thesis statement:

> Fumola programs enjoy **expressive, typed self-description** via symbolic
> names that carry meaning to and from human co-authors, with whom they
> co-evolve **shared symbolic meaning**.

Fumola is a programming language for humans describing programs that
describe themselves to themselves, over time.  To avoid the confusion
of this circular definition, Fumola uses a type system to control how
descriptions are mentioned and used.

#### Sharing meaning via shared symbol trees

The type system of Fumola uses a logic for **symbolic trees**, and these
trees' collective structure plays a key role in identifying data, computation, and its
interdependencies over time.

Unlike ordinary symbols, symbol trees give symbolic names that have
internal structure:
Fumola symbol trees compose and decompose according to simple, predictable rules.

These symbols' structure helps organize each subcomputation's effects
on the global store of symbolically-named, higher-order values.

To verify programs that use the store, the type system of Fumola
internalizes the structure of how symbolic names compose and decompose
while preserving their mutual distinctions (their individual
uniqueness, relative to some salient set of "sibling names").

Stored computations, when run (and re-run) produce (and incrementally
replace) stored program traces, the concrete data classified by store
effects.  The type and effect system is sound in the sense that these
effects always describe the program traces that they approximate
accurately.


## [Namesake](https://en.wikipedia.org/wiki/Namesake) notes

### <ins>Fu</ins>ngible

Thinking of

![image](https://user-images.githubusercontent.com/1183963/112759033-7716da80-8fae-11eb-917f-2cfeeebea3af.png)

Fungibles are provisional:
Fungibles stand in for other fungibles that replace them later.
Fungibles are placeholders for their future selves.

### <ins>Mo</ins>dels

Thinking of

> a program data **model** for program behavior.

And thinking of program models as program behavior traces.

Also thinking that we want to inspect a synthetic
["model organism"](https://en.wikipedia.org/wiki/Model_organism)
that embodies each system concept in a domain or system to be described.
 
### for <ins>L</ins>anguage <ins>A</ins>rts

Thinking of

- [Language_arts](https://en.wikipedia.org/wiki/Language_arts)

- [Programming_language](https://en.wikipedia.org/wiki/Programming_language)

- [The_Art_of_Computer_Programming](https://en.wikipedia.org/wiki/The_Art_of_Computer_Programming)
