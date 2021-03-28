# Fungible Models Language

Work in progress.

# Vision statement (draft)

The language where program behaviors on a global store (as effects) have a representation within the language's data model.

Fumola is a programming language for describing programs that describe themselves to themselves, over time.  To avoid the confusion of this circular defintion, Fumola uses a type system that classifies stored values (including both store names and stored names) and stored computations that have run, in a fine-grained way.  Only the latter have an associated data model representation for their (traced and retained) program behavior.

In Fumola, behavior becomes data in a systematic way, with refinement types that verify correct usage and avoid mismatched consumption-production linkage between programs that inspect behavior (data/behavior consumers) and programs that do the behavior (data/behavior producers).  In practical examples, programs mix these modes, and so Fumola uses general-purpose approach with type connectives for stored values, and for symbolic names of stored values.

The type system of Fumola internalizes aspects of a powerful, but specialized, effect system that describe approximations of each subcomputation's effects on the global store of named values.

To verify programs that use the store, the type system of Fumola internalizes the structure of how names compose and decompose while preserving their mutual distinctions (their individual uniqueness, relative to some salient set of "sibling names").  Stored computations, when run (and re-run) produce (and incrementally replace) stored program traces, the concrete data classified by store effects.  The type and effect system is sound in the sense that these effects always describe the program traces that they approximate accurately.

## "Fungible"

![image](https://user-images.githubusercontent.com/1183963/112759033-7716da80-8fae-11eb-917f-2cfeeebea3af.png)

## "Models"

Program behavior modeled with data.

https://en.wikipedia.org/wiki/Model_organism for each system concept in a domain or system to be described, but written as Fumola computations whose behavior becomes (model) data.
 
## "Language"

https://en.wikipedia.org/wiki/Programming_language
