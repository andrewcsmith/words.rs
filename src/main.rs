extern crate words;
extern crate astar;

use std::path::Path;
use words::{WordSearch, WordList, EspanolPalabras, EnglishWordList, WORDS_PATH};
use astar::astar;

fn main() {
    let mut args = std::env::args();
    args.next();
    let start = args.next().unwrap_or("".to_owned());
    let end = args.next().unwrap_or("".to_owned());
    let dict = args.next().unwrap_or(WORDS_PATH.to_owned());
    let words = EnglishWordList::new(Path::new(&dict)).expect("Could not initialize");
    let mut search = WordSearch::new(start, end, &words);
    let path = astar(&mut search).expect("A-star search failed.");
    for word in path {
        println!("{}", word.trim());
    }
}
