//! Levenshtein: A simple implementation of [Levenshtein distance]
//! (http://en.wikipedia.org/wiki/Levenshtein_distance) for strings.

#![crate_name = "levenshtein"]
#![crate_type = "lib"]

use std::cmp::min;

/// Count minimum number of single-character edits to turn one string into
/// another. Unicode-safe.
///
/// This implementation is based on C code by Sten Hjelmqvist [0].
///
/// 1. Hjelmqvist, Sten. (2012). [Fast, memory-efficient Levenshtein algorithm]
///    (http://www.codeproject.com/Articles/13525/Fast-memory-efficient-Levenshtein-algorithm).
pub fn levenshtein_dist<'a>(a: &'a str, b: &'a str) -> uint {
    // We need to deal with characters, not bytes --- so convert to characters.
    let achars: Vec<(uint, char)> = a.char_indices().collect();
    let bchars: Vec<(uint, char)> = b.char_indices().collect();
    
    // Degenerate cases
    if achars.len() == 0 { return bchars.len(); }
    if bchars.len() == 0 { return achars.len(); }

    // Just use current & previous rows to save memory
    let mut previous_row = Vec::from_fn(achars.len() + 1, |x| { x });
    let mut current_row = Vec::from_fn(achars.len() + 1, |x| { x });

    for row in range(1, bchars.len() + 1) {
        for (i, &value) in current_row.iter().enumerate() {
            *previous_row.get_mut(i) = value;
        }
        *current_row.get_mut(0) = row;

        for column in range(1, achars.len() + 1) {
            let cost = if achars[column - 1].val1() == bchars[row - 1].val1() { 0 } else { 1 };
            *current_row.get_mut(column) =
                min(min(current_row[column - 1] + 1, previous_row[column] + 1),
                    previous_row[column - 1] + cost);
        }
    }

    // To my knowledge, this should never fail
    match current_row.pop() {
        Some(val) => val,
        None => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::levenshtein_dist;

    #[test]
    fn test_dist() {
        assert_eq!(levenshtein_dist("kate", "cat"), 2);
        assert_eq!(levenshtein_dist("chantilly", "chandelier"), 5);
        assert_eq!(levenshtein_dist("soylent green is people", "people soiled our green"), 19);
    }

    #[test]
    fn test_unicode() {
        assert_eq!(levenshtein_dist("naïve", "œther"), 5);
    }

    #[bench]
    fn short_words_benchmark(b: &mut test::Bencher) {
        b.iter(|| {
            levenshtein_dist("kate", "cat")
        });
    }

    #[bench]
    fn long_words_benchmark(b: &mut test::Bencher) {
        b.iter(|| {
            levenshtein_dist("chantilly", "chandelier")
        });
    }

    #[bench]
    fn sentence_benchmark(b: &mut test::Bencher) {
        b.iter(|| {
            levenshtein_dist("soylent green is people", "people soiled our green")
        });
    }

    #[bench]
    fn unicode_benchmark(b: &mut test::Bencher) {
        b.iter(|| {
            levenshtein_dist("naïve", "œther")
        });
    }
}
