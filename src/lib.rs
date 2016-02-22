extern crate astar;
extern crate edit_distance;

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::vec::IntoIter;

use astar::SearchProblem;
use edit_distance::edit_distance;

static WORDS_PATH: &'static str = "/Users/acsmith/nltk_data/corpora/words/en";
static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn load_words() -> Result<Vec<String>, Error> {
    let file = try!(File::open(&WORDS_PATH));
    let file = BufReader::new(file);
    let out = file.lines().map(|line| {
        line.unwrap_or("".to_string()).to_lowercase()
    }).collect();
    Ok(out)
}

fn find_word(words: &Vec<String>, target: &String) -> Result<usize, usize> {
    words.binary_search(target)
}

fn adjacent_words(words: &Vec<String>, target: &String) -> Vec<String> {
    let mut out = Vec::<String>::new();

    for a in ALPHABET.chars() {
        // Create a new string with the one letter changed
        let mut insertion = String::with_capacity(target.len() + 1);
        insertion.push(a);
        for (pos, c) in target.char_indices() {
            insertion.push(c);
        };
        if &insertion != target && find_word(words, &insertion).is_ok() {
            out.push(insertion);
        }
    }

    for i in 0..target.len() {
        // Deletion
        let deletion = target.char_indices().filter_map(|(pos, c)| {
            if pos == i { None } else { Some(c) }
        }).collect();

        if &deletion != target && find_word(words, &deletion).is_ok() {
            out.push(deletion);
        }

        // Swap
        {
            let mut t = String::with_capacity(target.len());
            for (pos, c) in target.char_indices() {
                if pos > 0 && pos - 1 == i {
                    t.insert(i, c);
                } else {
                    t.push(c);
                }
            }

            if &t != target && find_word(words, &t).is_ok() {
                out.push(t);
            }
        }

        // Insertion
        for a in ALPHABET.chars() {
            // Create a new string with the one letter changed
            let mut insertion = String::with_capacity(target.len() + 1);
            for (pos, c) in target.char_indices() {
                insertion.push(c);
                if pos == i { insertion.push(a) }
            };
            if &insertion != target && find_word(words, &insertion).is_ok() {
                out.push(insertion);
            }
        }

        // Edit
        for a in ALPHABET.chars() {
            // Create a new string with the one letter changed
            let t = target.char_indices().map(|(pos, c)| {
                if pos == i { a } else { c }
            }).collect();

            if &t != target && find_word(words, &t).is_ok() {
                out.push(t);
            }
        }
    }
    out
}

pub struct WordSearch<'a> {
    start: String,
    end: String,
    words: &'a Vec<String>
}

impl<'a> SearchProblem for WordSearch<'a> {
    type Node = String;
    type Cost = i32;
    type Iter = IntoIter<(String, i32)>;

    fn start(&self) -> String {
        self.start.to_string()
    }

    fn is_end(&self, node: &String) -> bool {
        self.end == *node
    }

    fn heuristic(&self, node: &String) -> i32 {
        edit_distance(&self.end, &node) as i32
    }

    fn neighbors(&mut self, node: &String) -> IntoIter<(String, i32)> {
        let adj: Vec<String> = adjacent_words(&self.words, &node);
        let vec: Vec<(String, i32)> = adj.iter().map(|w| (w.to_string(), 1i32)).collect();
        vec.into_iter()
    }
}

#[test]
fn it_works() {
    let words = load_words().unwrap();
    println!("word #3: {}", words[2]);
    let find_me = "apple".to_string();
    println!("found apple at index {}", find_word(&words, &find_me).unwrap());
    let edit_me = "word".to_string();
    println!("Adjacent words: {:?}", adjacent_words(&words, &edit_me));
}

#[test]
fn test_path() {
    let inner_path = vec!["we", "member", "not", "the", "word", "but", "the", "sound", "of", "the", "word"];

    let words = load_words().unwrap();

    for a in inner_path.windows(2) {
        let mut search = WordSearch { 
            words: &words, 
            start: a[0].to_string(),
            end: a[1].to_string()
        };
        let path = astar::astar(&mut search).unwrap();
        println!("{}", a[0]);
        for word in path {
            println!("{}", word)
        }
    }
    panic!();
}

