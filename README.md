# levenshtein
Rust library to compute the Levenshtein distance between words or find the nearest word given a dictionnary.

See the [Wikipedia article](https://en.wikipedia.org/wiki/Levenshtein_distance)

# Example
```rust
use levenshtein::lev;
let a = String::from("Hello");
let b = String::from("World");
assert_eq!(4, lev(&a, &b));
```
