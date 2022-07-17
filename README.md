# Overly Fancy Tax Calculator
[![Rust](https://github.com/KuSpa/FluffyCodeForFluffyPeople/actions/workflows/rust.yml/badge.svg)](https://github.com/KuSpa/FluffyCodeForFluffyPeople/actions/workflows/rust.yml)

This is a small repo to solve the given coding challenge. 

### Problem Selection
I decided against the third problem as roman number parser are not interesting to implement.
I decided against the second problem as it NP-hard (by reduction from subsetsum for a single track). Of course, the problem is still solvable especially with these input sizes, but still ;).
So I took on the first Problem.

### Tech Stack Selection
As this task is solvable (and was solved) in O(n), the performance of the language was not relevant for this decision. Mainstream solutions would have been Python for it's fast coding speed speed and Java, because some people decided once that it should be industry standard and never admitted their mistake. 

I decided against Java because I neither like its String operations and management nor its Iterables mess. 
I decided against Python because I like statically typed languages.
Thus, I chose with ~~modern C++~~ Rust :).

### Additional Thoughts

* Float is not a good choice to represent decimal numbers. They get imprecise very quickly. A better choice would have been something like [bigdecimal](https://crates.io/crates/bigdecimal) which I found impractical to use. For production ready code I would either spend more time researching or write it myself.