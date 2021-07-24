# levenshtein
Rust library to compute the Levenshtein distance between words or find the nearest word given a dictionnary.

See the [Wikipedia article](https://en.wikipedia.org/wiki/Levenshtein_distance) and the algorithm used 
is [this one](https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm).
This actual algorithm was inspired by [wooorm](https://github.com/wooorm/levenshtein-rs).

## Comparing words
```rust
use levenshtein::lev;
let a = "Hello";
let b = "World";
assert_eq!(4, lev(a, b));
```

## Finding the nearest word in a dictionnary
Example dictionnary can be found in [this repo](https://github.com/dwyl/english-words).

You can download a dictionnary and try the example called `didyoumean` like this
```bash
cargo run --release --example didyoumean <a word> <a dictionnary filename>
```

## Online demo
See [there](https://test.alain.dev) for a demo (with a french dictionnary).
