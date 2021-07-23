# levenshtein
Rust library to compute the Levenshtein distance between words or find the nearest word given a dictionnary.

See the [Wikipedia article](https://en.wikipedia.org/wiki/Levenshtein_distance) and the algorithm used 
is [this one](https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm).

# Comparing words
```rust
use levenshtein::lev;
let a = "Hello";
let b = "World";
assert_eq!(4, lev(a, b));
```

# Finding the nearest word in a dictionnary
Example dictionnary can be found in [this repo.](https://github.com/dwyl/english-words).
