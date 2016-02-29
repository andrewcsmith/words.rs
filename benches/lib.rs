#![feature(test)]
extern crate test;

extern crate words;
extern crate astar;
extern crate edit_distance;

use words::*;
use test::Bencher;
use std::path::Path;

#[bench]
fn bench_adjacent_word1(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "remember";
    b.iter(|| words.adjacent_words(&a, 1));
}

#[bench]
fn bench_adjacent_word2(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "remember";
    b.iter(|| words.adjacent_words(&a, 2));
}

#[bench]
fn bench_adjacent_word4(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "remember";
    b.iter(|| words.adjacent_words(&a, 4));
}

#[bench]
fn bench_adjacent_word8(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "remember";
    b.iter(|| words.adjacent_words(&a, 8));
}

#[bench]
fn bench_adjacent_word16(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "remember";
    b.iter(|| words.adjacent_words(&a, 16));
}

#[bench]
fn bench_find_word(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let a = "remember";
    b.iter(|| words.find_word(&a));
}

#[bench]
fn bench_edit_distance_len8(b: &mut Bencher) {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let start = "remember";
    let target = "language";
    let adjacent = words.adjacent_words(&start, 8);
    b.iter(|| {
        for ref w in &adjacent {
            edit_distance::edit_distance(&w, &target);
        }
    });
}
