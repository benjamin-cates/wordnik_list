//! This is a small library that exports useful methods for recognizing valid words from the [Wordnik Word List](https://developer.wordnik.com). Definitions, usage, pronunciation, etymology, and examples are omitted.
//! This library uses the official list from wordnik (last synced December 2024). Internally, it uses
//! a static BTreeSet generated at runtime, meaning cross-thread word checking in a small footprint
//! under 10 MB.
//!
//! ## Startup runtime
//! For the first lookup, generating the map takes around 13 milliseconds in release mode on a mid-range desktop in 2024.
//! For debug mode, initial lookup took around 45 milliseconds.
//! ## Random lookup time
//! Runtime is around 300 nanoseconds per lookup in release mode on a mid-range desktop in 2024.
//! For debug mode, runtime per lookup is around 900 nanoseconds.
//!
//! ## Stability note
//! This crate may change implementation at any time to improve memory efficiency or startup time, 
//! but main exported functions should still be there.
//!
//! Example:
//! ```rust
//! use wordnik_list::word_exists;
//!
//! let try_words = ["list", "rust", "rusty", "ruster", "abroptly", "assertion"];
//! // Print which words in that list are invalid
//! for word in try_words {
//!     if !word_exists(word) {
//!         println!("\"{}\" is not a valid word", word);
//!     }
//! }
//! ```
//!
//! Another example:
//! ```rust
//! use wordnik_list::word_iterator;
//!
//! // Collect list of every 2-letter word
//! let vec: Vec<&str> = word_iterator().filter(|word| word.len() == 2).collect();
//! println!("List of every 2-letter word: {:?}", vec);
//! ```
use std::{collections::BTreeSet, ops::Bound, sync::OnceLock};

static DICT_SINGLETON: OnceLock<BTreeSet<&'static str>> = OnceLock::new();

/// Accepts a lowercase ASCII encoded string reference and returns whether it is a valid word or not.
/// Note: this will always fail if there are any characters outside of the lowercase range \[a-z\].
///
/// Example:
/// ```rust
/// use wordnik_list::word_exists;
/// assert!(word_exists("rusty"));
/// ```
pub fn word_exists(word: &str) -> bool {
    get_dict().contains(word)
}

/// Returns an alphabetically sorted iterator of the words in the range [begin, end).
///
/// Example:
/// ```rust
/// use wordnik_list::word_range;
/// // Counts the number of words between "aa" and "ab" (not including "ab").
/// assert_eq!(word_range("aa", "ab").count(), 20);
/// ```
pub fn word_range<'a>(begin: &'a str, end: &'a str) -> impl Iterator<Item = &'a str> {
    get_dict()
        .range::<&'a str, _>((Bound::Included(begin), Bound::Excluded(end)))
        .cloned()
}

fn get_dict() -> &'static BTreeSet<&'static str> {
    const TEXT_CONTENT: &str = include_str!("wordlist.txt");
    DICT_SINGLETON.get_or_init(|| TEXT_CONTENT.lines().collect())
}

/// Returns an iterator over every valid word registered in the Wordnik API.
/// 
/// Words are represented by static references (`&'static str`)
///
/// Note: Calling this function for the first time will compute the set and may incur some overhead.
/// However, subsequent calls will be negligible.
///
/// Example:
/// ```
/// use wordnik_list::word_iterator;
/// assert!(word_iterator().count() > 0);
/// ```
pub fn word_iterator() -> impl Iterator<Item = &'static str> {
    get_dict().iter().cloned()
}

#[cfg(test)]
mod tests {
    use crate::{word_exists, word_iterator, word_range};

    #[test]
    fn test_size() {
        let now = std::time::Instant::now();
        assert_eq!(word_iterator().count(), 198424);
        println!("Initialization took: {:?}", now.elapsed());
    }

    #[test]
    fn test_words_exist() {
        assert!(word_exists("zebra"));
        assert!(word_exists("a"));
        assert!(word_exists("ab"));
        assert!(word_exists("an"));
        assert!(word_exists("the"));
    }

    #[test]
    fn test_words_dont_exit() {
        assert!(!word_exists("asd"));
        assert!(!word_exists("zzzzzz"));
        assert!(!word_exists("1ab"));
    }
    
    #[test]
    fn test_word_range() {
        // There are 95 3-letter words starting with "a"
        assert_eq!(word_range("a", "b").filter(|word| word.len() == 3).count(), 95);
    }

    #[test]
    fn randomized_reading() {
        let word_vec: Vec<&'static str> = word_iterator().collect();
        let now = std::time::Instant::now();
        for i in 0..1 {
            assert!(word_exists(word_vec[(i * 80) % word_vec.len()]));
        }
        println!("Ranodom iteration lookup: {:?}", now.elapsed().div_f32(1.0));
    }
}
