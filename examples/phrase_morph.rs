extern crate words;
extern crate astar;

use std::path::Path;
use words::*;

fn main() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let start = "not".to_string();
    let end = "the".to_string();
    let mut search = WordSearch::new(start, end, &words);
    astar::astar(&mut search);
}
