extern crate words;
extern crate astar;

use std::path::Path;
use words::{WordSearch, WordList, EspanolPalabras, EnglishWordList, WORDS_PATH};
use astar::astar;

fn main() {
    let mut args = std::env::args();
    args.next();
    let start = args.next().unwrap_or("".to_owned());
    println!("{}", start);
    let dict = args.next().unwrap_or(WORDS_PATH.to_owned());
    let words = EnglishWordList::new(Path::new(&dict)).expect("Could not initialize");
    let search = WordSearch::new(start.clone(), String::new(), &words);
    let list = search.adjacent_words(&start.to_owned(), 1);
    for word in list {
        println!("{}", word.trim());
    }
}
