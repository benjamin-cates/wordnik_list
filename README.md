# Wordnik List in Rust

A quick, local library to query valid English words.

## Description
This is a small library that exports useful methods for recognizing valid words from the [Wordnik Word List](https://developer.wordnik.com). Definitions, usage, pronunciation, etymology, and examples are omitted.
This library uses the official list from wordnik (last synced December 2024). Internally, it uses
a static BTreeSet generated at runtime, meaning cross-thread word checking in a small footprint
under 10 MB.

## Installation
Add to your Rust project with `cargo add wordnik-list`

## Startup runtime
For the first lookup, generating the map takes around 13 milliseconds in release mode on a mid-range desktop in 2024.
For debug mode, initial lookup took around 45 milliseconds.
## Random lookup time
Runtime is around 300 nanoseconds per lookup in release mode on a mid-range desktop in 2024.
For debug mode, runtime per lookup is around 900 nanoseconds.

## Stability note
This crate may change implementation at any time to improve memory efficiency or startup time, 
but main exported functions should still be there.

Example:
```rust
use wordnik_list::word_exists;

let try_words = ["list", "rust", "rusty", "ruster", "abroptly", "assertion"];
// Print which words in that list are invalid
for word in try_words {
    if !word_exists(word) {
        println!("\"{}\" is not a valid word", word);
    }
}
```

Another example:
```rust
use wordnik_list::word_iterator;

// Collect list of every 2-letter word
let vec: Vec<&str> = word_iterator().filter(|word| word.len() == 2).collect();
println!("List of every 2-letter word: {:?}", vec);
```


## Changelog

### Version 0.2.0
- Added new function: `word_iterator_by_len`
- Removed guarantee that iterators returned by word_iterator and word_range will be in alphabetical order.
- Improved internal implementation: replaced BTreeMap with a combined string per word length.
    - Runtime memory usage reduced by about 50%.
    - Performance penalty at first call removed.
    - Word lookup time improved

### Version 0.1.0
- Initial release with 3 functions: `word_iterator`, `word_exists`, and `word_range`.
- Based on BTreeMap implementation that is computed at first call