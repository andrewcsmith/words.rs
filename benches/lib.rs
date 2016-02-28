#![feature(test)]

extern crate words;
extern crate test;

use words::*;
use test::Bencher;
use std::path::Path;

/// test bench_find_word ... bench:   8,055,412 ns/iter (+/- 2,199,765)
#[bench]
fn bench_adjacent_word1(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "we remember not the word";
    b.iter(|| words.adjacent_words(&a, 1));
}

#[bench]
fn bench_adjacent_word2(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "we remember not the word";
    b.iter(|| words.adjacent_words(&a, 2));
}

#[bench]
fn bench_adjacent_word4(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "we remember not the word";
    b.iter(|| words.adjacent_words(&a, 4));
}

#[bench]
fn bench_adjacent_word8(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "we remember not the word";
    b.iter(|| words.adjacent_words(&a, 8));
}

#[bench]
fn bench_adjacent_word16(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "we remember not the word";
    b.iter(|| words.adjacent_words(&a, 16));
}

#[bench]
fn bench_find_word(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "remember";
    b.iter(|| words.find_word(&a));
}

