extern crate astar;
extern crate edit_distance;

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::vec::IntoIter;
use std::path::Path;

use astar::SearchProblem;
use edit_distance::edit_distance;

static WORDS_PATH: &'static str = "/Users/acsmith/nltk_data/corpora/words/en";
static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz ";

pub struct WordList {
    words: Vec<String>
}

impl WordList {
    pub fn new(path: &Path) -> Result<WordList, Error> {
        let file = try!(File::open(&path));
        let file = BufReader::new(file);
        let out = file.lines().map(|line| {
            line.unwrap_or("".to_string()).to_lowercase()
        }).collect();
        Ok(WordList{ words: out})
    }

    fn find_word(&self, target: &str) -> Result<usize, usize> {
        self.words.binary_search_by(|w| w[..].cmp(target))
    }

    fn insert_if_new<'a>(&'a self, out: &'a mut Vec<String>, word: String, target: &str) {
        if word == *target { return; }
        if word.split_whitespace().all(|w| {
            self.find_word(w).is_ok()
        }) { out.push(word); }
    }

    fn adjacent_words(&self, target: &str) -> Vec<String> {
        let mut out = Vec::<String>::new();

        for a in ALPHABET.chars() {
            let mut insertion = String::with_capacity(target.len() + 1);
            insertion.push(a);
            for (pos, c) in target.char_indices() {
                insertion.push(c);
            };

            self.insert_if_new(&mut out, insertion, target);
        }

        for i in 0..target.len() {
            // Deletion
            let deletion = target.char_indices().filter_map(|(pos, c)| {
                if pos == i { None } else { Some(c) }
            }).collect();

            self.insert_if_new(&mut out, deletion, target);

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

                self.insert_if_new(&mut out, t, target);
            }

            // Edit
            for a in ALPHABET.chars() {
                // Create a new string with the one letter changed
                let t = target.char_indices().map(|(pos, c)| {
                    if pos == i { a } else { c }
                }).collect();

                self.insert_if_new(&mut out, t, target);
            }

            // Insert
            for a in ALPHABET.chars() {
                let mut insertion = String::with_capacity(target.len() + 1);
                for (pos, c) in target.char_indices() {
                    insertion.push(c);
                    if pos == i { insertion.push(a) }
                };

                self.insert_if_new(&mut out, insertion, target);
            }

        }
        out
    }
}

pub struct WordSearch<'a> {
    start: String,
    end: String,
    words: &'a WordList
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
        let adj: Vec<String> = self.words.adjacent_words(&node);
        let vec: Vec<(String, i32)> = adj.iter().map(|w| (w.to_string(), 1i32)).collect();
        vec.into_iter()
    }
}

#[test]
fn test_path() {
    // let inner_path = vec!["we", "member", "not", "the", "word", "but", "the", "sound", "of", "the", "word"];
    let inner_path = vec!["we", "not"];

    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();

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
}

#[test]
fn test_adjacent_words() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let w = "rata";
    let adjacent = words.adjacent_words(&w.to_string());
    println!("{:?}", adjacent);
    // Ensure insertion works
    assert!(adjacent.iter().find(|a| *a == &"ratwa".to_string()).is_some());
    // Ensure deletion works
    assert!(adjacent.iter().find(|a| *a == &"rat".to_string()).is_some());
}

#[test]
fn test_inserts_space() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let w = "treehouse";
    let adjacent = words.adjacent_words(&w.to_string());
    println!("{:?}", adjacent);
    // Ensure insertion works
    assert!(adjacent.iter().find(|a| *a == &"tree house".to_string()).is_some());
}

#[test]
fn test_insert_if_new() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let mut out = Vec::<String>::new();
    let w = "tree house".to_string();
    let result = words.insert_if_new(&mut out, w, &"treehouse".to_string());
    println!("{:?}", &out);
    assert!(out.iter().find(|a| *a == &"tree house".to_string()).is_some());
}

#[test]
fn test_deletes_space() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let w = "re member";
    let adjacent = words.adjacent_words(&w.to_string());
    println!("{:?}", adjacent);
    // Ensure deletion works
    assert!(adjacent.iter().find(|a| *a == &"remember".to_string()).is_some());
}

#[test]
fn test_mult_words() {
    let start = "remember";
    let end = "not";
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let mut search = WordSearch { words: &words, start: start.to_string(), end: end.to_string() };
    match astar::astar(&mut search) {
        Some(path) => {
            for word in &path {
                println!("{}", word);
            }
            assert_eq!(path.len(), 10);
        }
        None => { panic!("Could not resolve path"); }
    }
}

