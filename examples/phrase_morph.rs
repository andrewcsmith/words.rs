extern crate words;
extern crate astar;

use std::path::Path;
use words::{WordSearch, WordList, WORDS_PATH};
use astar::astar;

fn main() {
    let mut args = std::env::args();
    args.next();
    let start = args.next().unwrap_or("".to_owned());
    let end = args.next().unwrap_or("".to_owned());
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let mut search = WordSearch::new(start, end, &words);
    let path = astar(&mut search).unwrap();
    for word in path {
        println!("{}", word);
    }
}
