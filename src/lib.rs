//! This is a small library that exports useful methods for recognizing valid words from the [Wordnik Word List](https://developer.wordnik.com). Definitions, usage, pronunciation, etymology, and examples are omitted.
//! This library uses the official list from wordnik (last synced December 2024). Internally, it uses
//! a static BTreeSet generated at runtime, meaning cross-thread word checking in a small footprint
//! under 10 MB.
//!
//! ## Random lookup time
//! Averaged over 100,000 cases on a mdi-range desktop in 2024, the runtime is 200 nanoseconds in release mode and 600 nanoseconds in debug mode.
//!
//! ## Stability note
//! This crate may change internal implementation at any time to improve memory efficiency or startup time.
//! Do not rely on the order that elements are yielded by iterators
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

#![no_std]

const LEN_MAPS: [&'static str; 29] = [
    "",
    "",
    include_str!("words/len_2.txt"),
    include_str!("words/len_3.txt"),
    include_str!("words/len_4.txt"),
    include_str!("words/len_5.txt"),
    include_str!("words/len_6.txt"),
    include_str!("words/len_7.txt"),
    include_str!("words/len_8.txt"),
    include_str!("words/len_9.txt"),
    include_str!("words/len_10.txt"),
    include_str!("words/len_11.txt"),
    include_str!("words/len_12.txt"),
    include_str!("words/len_13.txt"),
    include_str!("words/len_14.txt"),
    include_str!("words/len_15.txt"),
    include_str!("words/len_16.txt"),
    include_str!("words/len_17.txt"),
    include_str!("words/len_18.txt"),
    include_str!("words/len_19.txt"),
    include_str!("words/len_20.txt"),
    include_str!("words/len_21.txt"),
    include_str!("words/len_22.txt"),
    include_str!("words/len_23.txt"),
    include_str!("words/len_24.txt"),
    include_str!("words/len_25.txt"),
    include_str!("words/len_26.txt"),
    include_str!("words/len_27.txt"),
    include_str!("words/len_28.txt"),
];

/// On success, returns Ok(idx) of the starting index of the string in the haystack
/// On fail, returns Err(idx) of the index where the string should be inserted
/// In the case that the needle is greater than every string in the haystack, it will return haystack.len()
fn str_binary_search(haystack: &str, needle: &str, len: usize) -> Result<usize, usize> {
    let mut start = 0;
    let mut end = haystack.len() / len;
    while start != end {
        let mid = (end - start) / 2 + start;
        let middle_word = &haystack[mid * len..(mid + 1) * len];
        if needle == middle_word {
            return Ok(mid * len);
        } else if needle > middle_word {
            start = mid + 1;
        } else if needle < middle_word {
            end = mid;
        }
    }
    Err(start * len)
}

/// Accepts a lowercase ASCII encoded string reference and returns whether it is a valid word or not.
/// Note: this will always fail if there are any characters outside of the lowercase range \[a-z\].
///
/// Example:
/// ```rust
/// use wordnik_list::word_exists;
/// assert!(word_exists("rusty"));
/// assert!(!word_exists("rustying"));
/// ```
pub fn word_exists(word: &str) -> bool {
    let word_len = word.len();
    if word.len() < 2 {
        return false;
    }
    // Get list of valid words that length
    let list = *LEN_MAPS.get(word_len).unwrap_or(&"");
    // Perform binary search on the string list with uniform length
    str_binary_search(list, word, word_len).is_ok()
}

/// Returns an iterator of the valid words in the range [begin, end).
/// Not guaranteed to return them in any particular order.
///
/// Example:
/// ```rust
/// use wordnik_list::word_range;
/// // Counts the number of words between "aa" and "ab" (not including "ab").
/// assert_eq!(word_range("aa", "ab").count(), 20);
/// ```
pub fn word_range<'a>(
    begin: &'a str,
    end: &'a str,
) -> impl Iterator<Item = &'static str> + use<'a> {
    let mut len = 1;
    let mut index = 0;
    core::iter::from_fn(move || {
        if len > 28 {
            return None;
        }
        index += len;
        while LEN_MAPS[len].len() <= index || &LEN_MAPS[len][index..index + len] >= end {
            len += 1;
            if len > 28 {
                return None;
            }
            index = match str_binary_search(LEN_MAPS[len], begin, len) {
                Ok(pos) => pos,
                Err(pos) => pos,
            };
        }
        Some(&LEN_MAPS[len][index..index + len])
    })
}

/// Returns an iterator over every valid word registered in the Wordnik API. Yield order is not guaranteed
///
/// Note: This iterator yields around 200,000 elements, so use it with caution. On release mode, this iterator takes around 1 ms just to count.
///
/// For a more optimized use case, use [`word_iterator_by_len`].
///
/// Example:
/// ```
/// use wordnik_list::word_iterator;
/// assert!(word_iterator().count() > 0);
/// ```
pub fn word_iterator() -> impl Iterator<Item = &'static str> {
    let mut len = 1;
    let mut index = 0;
    core::iter::from_fn(move || {
        if len > 28 {
            return None;
        }
        index += len;
        while LEN_MAPS[len].len() <= index {
            len += 1;
            if len > 28 {
                return None;
            }
            index = 0;
        }
        Some(&LEN_MAPS[len][index..index + len])
    })
}

/// Returns an iterator of every valid word with a given length registered in the Wordnik API.
///
/// For an iterator over every word, use [`word_iterator`].
///
/// Example:
/// ```rust
/// use wordnik_list::word_iterator_by_len;
/// // Get all 3 letter words
/// let vec: Vec<&str> = word_iterator_by_len(3).collect();
/// ```
pub fn word_iterator_by_len(len: usize) -> impl Iterator<Item = &'static str> {
    let mut list = *LEN_MAPS.get(len).unwrap_or(&"");
    core::iter::from_fn(move || {
        if list.len() == 0 {
            return None;
        }
        let out = &list[0..len];
        list = &list[len..];
        Some(out)
    })
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    extern crate std;
    use crate::{str_binary_search, word_exists, word_iterator, word_iterator_by_len, word_range};

    #[test]
    fn test_size() {
        let now = std::time::Instant::now();
        assert_eq!(word_iterator().count(), 198420);
        std::println!("Count all took: {:?}", now.elapsed());
    }

    #[test]
    fn test_words_exist() {
        assert!(word_exists("zebra"));
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
    fn test_odd_inputs() {
        assert_eq!(word_range("z", "a").next(), None);
        assert_eq!(word_range("~", "~").next(), None);
        assert_eq!(word_range("aa", "aa").next(), None);
        assert_eq!(word_iterator_by_len(0).next(), None);
        assert_eq!(word_iterator_by_len(1).next(), None);
        assert_eq!(word_iterator_by_len(29).next(), None);
        assert_eq!(word_exists(""), false);
        assert_eq!(word_exists("a"), false);
        assert_eq!(word_exists("~"), false);
    }

    #[test]
    fn test_word_range() {
        // There are 95 3-letter words starting with "a"
        assert_eq!(
            word_range("a", "b").filter(|word| word.len() == 3).count(),
            95
        );
    }

    #[test]
    fn randomized_reading() {
        let word_vec: std::vec::Vec<&'static str> = word_iterator().collect();
        let now = std::time::Instant::now();
        for i in 0..100000 {
            assert!(word_exists(word_vec[(i * 80) % word_vec.len()]));
        }
        std::println!("Random iteration lookup: {:?}", now.elapsed().div_f32(100000.0));
    }
    #[test]
    fn test_str_binary_search() {
        let haystack = "bcef";
        assert_eq!(str_binary_search(haystack, "a", 1), Err(0));
        assert_eq!(str_binary_search(haystack, "b", 1), Ok(0));
        assert_eq!(str_binary_search(haystack, "c", 1), Ok(1));
        assert_eq!(str_binary_search(haystack, "d", 1), Err(2));
        assert_eq!(str_binary_search(haystack, "e", 1), Ok(2));
        assert_eq!(str_binary_search(haystack, "f", 1), Ok(3));
        assert_eq!(str_binary_search(haystack, "g", 1), Err(4));
        let haystack = "babbbccdef";
        assert_eq!(str_binary_search(haystack, "aa", 2), Err(0));
        // Testing with a string that's too short
        assert_eq!(str_binary_search(haystack, "b", 2), Err(0));
        assert_eq!(str_binary_search(haystack, "ba", 2), Ok(0));
        // Testing with a string that's too long
        assert_eq!(str_binary_search(haystack, "baa", 2), Err(2));
        assert_eq!(str_binary_search(haystack, "bb", 2), Ok(2));
        assert_eq!(str_binary_search(haystack, "bc", 2), Ok(4));
        assert_eq!(str_binary_search(haystack, "ca", 2), Err(6));
        assert_eq!(str_binary_search(haystack, "cd", 2), Ok(6));
        assert_eq!(str_binary_search(haystack, "ee", 2), Err(8));
        assert_eq!(str_binary_search(haystack, "ef", 2), Ok(8));
        assert_eq!(str_binary_search(haystack, "zz", 2), Err(10));
    }
}
