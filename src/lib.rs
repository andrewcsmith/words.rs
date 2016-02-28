extern crate astar;
extern crate edit_distance;
extern crate simple_parallel;

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::iter::Iterator;
use std::path::Path;

use astar::SearchProblem;
use edit_distance::edit_distance;
use simple_parallel::Pool;

pub static WORDS_PATH: &'static str = "./data/en";
pub static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz ";

pub struct WordList {
    words: Vec<String>
}

impl WordList {
    pub fn new(path: &Path) -> Result<WordList, Error> {
        let file = try!(File::open(&path));
        let file = BufReader::new(file);
        let out = file.lines().map(|line| {
            line.unwrap_or("".to_owned()).to_lowercase()
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

    pub fn adjacent_words(&self, target: &str) -> Vec<String> {
        let capacity = ALPHABET.len() * target.len() * 3 + ALPHABET.len() + target.len();
        let mut out = Vec::<String>::with_capacity(capacity);

        for a in ALPHABET.chars() {
            let mut insertion = String::with_capacity(target.len() + 1);
            insertion.push(a);
            for c in target.chars() {
                insertion.push(c);
            };

            self.insert_if_new(&mut out, insertion, target);
        }

        let mut local_vecs = Vec::<Vec<String>>::with_capacity(target.len());
        for _ in 0..target.len() {
            local_vecs.push(Vec::<String>::with_capacity(ALPHABET.len() * 2 + 2));
        }

        let mut pool = Pool::new(4);

        pool.for_(local_vecs.iter_mut(), |mut local_vec| {
            for i in 0..target.len() {
                {
                    let mut t = String::with_capacity(target.len()-1);
                    for (pos, c) in target.char_indices() {
                        if pos != i { t.push(c); }
                    }

                    self.insert_if_new(&mut local_vec, t, target);
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

                    self.insert_if_new(&mut local_vec, t, target);
                }

                // Edit
                for a in ALPHABET.chars() {
                    let mut t = String::with_capacity(target.len());
                    // Create a new string with the one letter changed
                    for (pos, c) in target.char_indices() {
                        if pos == i {
                            t.push(a);
                        } else {
                            t.push(c);
                        }
                    }

                    self.insert_if_new(&mut local_vec, t, target);
                }

                // Insert
                for a in ALPHABET.chars() {
                    let mut insertion = String::with_capacity(target.len() + 1);
                    for (pos, c) in target.char_indices() {
                        insertion.push(c);
                        if pos == i { insertion.push(a) }
                    };

                    self.insert_if_new(&mut local_vec, insertion, target);
                }
            }
        });
        for ref mut vec in local_vecs { out.append(vec); }
        out
    }
}

pub struct WordSearch<'a> {
    start: String,
    end: String,
    words: &'a WordList
}

impl<'a> WordSearch<'a> {
    pub fn new(start: String, end: String, words: &'a WordList) -> WordSearch {
        WordSearch {
            start: start,
            end: end,
            words: words
        }
    }
}

impl<'a> SearchProblem for WordSearch<'a> {
    type Node = String;
    type Cost = i32;
    type Iter = Box<Iterator<Item=(String, i32)>>;

    fn start(&self) -> String {
        self.start.to_owned()
    }

    fn is_end(&self, node: &String) -> bool {
        self.end == *node
    }

    fn heuristic(&self, node: &String) -> i32 {
        edit_distance(&self.end, &node) as i32
    }

    fn neighbors(&mut self, node: &String) -> Box<Iterator<Item=(String, i32)>> {
        let adj: Vec<String> = self.words.adjacent_words(&node);
        Box::new(adj.into_iter().map(|w| (w, 1i32)))
    }
}

#[test]
fn test_path() {
    // let inner_path = vec!["we", "member", "not", "the", "word", "but", "the", "sound", "of", "the", "word"];
    let inner_path = vec!["we", "not"];

    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();

    for a in inner_path.windows(2) {
        let mut search = WordSearch::new(a[0].to_owned(), a[1].to_owned(), &words);
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
    let adjacent = words.adjacent_words(&w.to_owned());
    println!("{:?}", adjacent);
    // Ensure insertion works
    assert!(adjacent.iter().find(|a| *a == &"ratwa".to_owned()).is_some());
    // Ensure deletion works
    assert!(adjacent.iter().find(|a| *a == &"rat".to_owned()).is_some());
}

#[test]
fn test_inserts_space() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let w = "treehouse";
    let adjacent = words.adjacent_words(&w.to_owned());
    println!("{:?}", adjacent);
    // Ensure insertion works
    assert!(adjacent.iter().find(|a| *a == &"tree house".to_owned()).is_some());
}

#[test]
fn test_insert_if_new() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let mut out = Vec::<String>::new();
    let w = "tree house".to_owned();
    let result = words.insert_if_new(&mut out, w, &"treehouse".to_owned());
    println!("{:?}", &out);
    assert!(out.iter().find(|a| *a == &"tree house".to_owned()).is_some());
}

#[test]
fn test_deletes_space() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let w = "re member";
    let adjacent = words.adjacent_words(&w.to_owned());
    println!("{:?}", adjacent);
    // Ensure deletion works
    assert!(adjacent.iter().find(|a| *a == &"remember".to_owned()).is_some());
}

#[test]
fn test_mult_words() {
    let start = "remember";
    let end = "not";
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let mut search = WordSearch { words: &words, start: start.to_owned(), end: end.to_owned() };
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

