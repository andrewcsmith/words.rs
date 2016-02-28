#![feature(test)]

extern crate words;
extern crate test;

use words::*;
use test::Bencher;
use std::path::Path;

/// test bench_find_word ... bench:   8,442,916 ns/iter (+/- 3,298,273)
#[bench]
fn bench_find_word(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "we remember not the word";
    b.iter(|| words.adjacent_words(&a));
}

