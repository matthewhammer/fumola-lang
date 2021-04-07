# Fungible Models Language

## Summary

Fumola programs consist of networks of processes that operate over a shared symbolic space, in shared symbolic  time.
Their collective behavior on the shared space-time consists of four symbolic effects, each attributed to a single process in the network, as it interacts with itself, and others:

#### Put operation
A put operation has the effect of addressing a stored a value, and producing that address for subsequent get operations.

#### Link operation
A link operation has the effect of securing the address of a stored value with a query, where the value's precise type and address may not be available, but perhaps only partially known.

#### Get operation
A get operation retrieves the current stored value of a symbolic address.

#### Split operation

Finally, processes can subdivide their own control structure, creating forked sub-processes (their child processes).  We call this operation splitting, but "forking" or "spawning" are equally good descriptive words.

Each operation creates a traced record, as a global history.  The network cooperates to process and curate this history, whose access is controlled with a type and effect system, a la a capabilities-based security model.

In each case, the symbolic data of the past within the network becomes input for future process behavior (via link and get operations), which creates (yet) more symbolic trace data to process in the future.

## Some details (preview)

Fumola is a programming language for describing programs that describe
themselves to themselves, over time.

To avoid the confusion of this circular definition, Fumola uses a type
system to control how descriptions are mentioned and used.

The type system of Fumola uses a logic for symbolic trees, and these trees play a key role
in identifying data, computation, and its interdependencies over time.

Unlike ordinary symbols, symbol trees give symbolic names that have
internal structure (trees compose and decompose).

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

## Namesake

### Fungible

![image](https://user-images.githubusercontent.com/1183963/112759033-7716da80-8fae-11eb-917f-2cfeeebea3af.png)

### Models

Program behavior modeled with data.

https://en.wikipedia.org/wiki/Model_organism for each system concept in a domain or system to be described, but written as Fumola computations whose behavior becomes (model) data.
 
### for Language Arts

- https://en.wikipedia.org/wiki/Language_arts

- https://en.wikipedia.org/wiki/Programming_language

- https://en.wikipedia.org/wiki/The_Art_of_Computer_Programming
