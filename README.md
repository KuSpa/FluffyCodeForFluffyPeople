# Overly Fancy Tax Calculator
[![test](https://github.com/KuSpa/FluffyCodeForFluffyPeople/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/KuSpa/FluffyCodeForFluffyPeople/actions/workflows/rust.yml) [![build](https://github.com/KuSpa/FluffyCodeForFluffyPeople/actions/workflows/build.yml/badge.svg)](https://github.com/KuSpa/FluffyCodeForFluffyPeople/actions/workflows/build.yml)

This is a small repo to solve the given coding challenge. 

### Run
If you want to compile the code yourself, please install rust using[ this guide](https://www.rust-lang.org/tools/install). Otherwise, you can find artifacts for the major os' attached to the releases.

The program expects a file containing the shopping list as input. Example:

```
./itemis testFiles/1
```


### Problem Selection
I decided against the third problem as roman number parser are not interesting to implement.
I decided against the second problem as it NP-hard (by reduction from subsetsum for a single track). Of course, the problem is still solvable especially with these input sizes, but still ;).
So I took on the first Problem.

### Tech Stack Selection
As this task is solvable (and was solved) in O(n), the performance of the language was not relevant for this decision. Mainstream solutions would have been Python for it's fast coding speed speed and Java, because some people decided once that it should be industry standard and never admitted their mistake. 

I decided against Java because I neither like its String operations and management nor its Iterables mess. 
I decided against Python because I like statically typed languages.
Thus, I chose with ~~modern C++~~ Rust :).

### CI/CD
There is a number of unit tests, that need to run in order to merge into the main branch. Additionally, artifacts for every major os are build and added to the release automatically.

### Additional Thoughts

* Float is not a good choice to represent decimal numbers. They get imprecise very quickly. A better choice would have been something like [bigdecimal](https://crates.io/crates/bigdecimal) which I found impractical to use. For production ready code I would either spend more time researching or write it myself.