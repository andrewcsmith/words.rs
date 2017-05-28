extern crate astar;
extern crate edit_distance;
extern crate simple_parallel;
extern crate pipers;

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::iter::Iterator;
use std::path::Path;
use std::collections::HashSet;
use std::process::Command;

use astar::SearchProblem;
use edit_distance::edit_distance;
use simple_parallel::Pool;

pub static WORDS_PATH: &'static str = "./data/es_large";
// pub static ALPHABET: &'static str = "abcdefghijklmnñopqrstuvwxyz ";
pub static ALPHABET: &'static str = "aáàbcdeéèfghiíìjklmnñoópqrstuúùvwxyz ";
// pub static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz ";

pub struct EnglishWordList {
    words: HashSet<String>
}

pub struct EspanolPalabras;

pub trait WordList: Sync {
    fn find_word(&self, target: &str) -> bool;
}

impl EnglishWordList {
    pub fn new(path: &Path) -> Result<EnglishWordList, Error> {
        let file = try!(File::open(&path));
        let file = BufReader::new(file);
        let mut out: HashSet<String> = HashSet::new();
        for line in file.lines() {
            out.insert(line.unwrap_or("".to_owned()).to_lowercase());
        }
        Ok(EnglishWordList{ words: out })
    }
}

impl WordList for EnglishWordList {
    fn find_word(&self, target: &str) -> bool {
        self.words.contains(target)
    }
}

impl WordList for EspanolPalabras {
    fn find_word(&self, target: &str) -> bool {
        let out = pipers::Pipe::new(format!("echo {}", target).as_str())
            .then("aspell --lang=es list")
            .finally()
            .expect("Commands did not pipe.")
            .wait_with_output()
            .expect("Failed to wait on child.");

        let bad_words = String::from_utf8(out.stdout).expect("stdout conversion failed.");
        // println!("{}", bad_words.trim());
        if bad_words.len() > 0 { false } else { true }
    }
}

pub struct WordSearch<'a, WL> 
where WL: 'a + WordList
{
    start: String,
    end: String,
    words: &'a WL
}

impl<'a, WL> WordSearch<'a, WL> 
where WL: 'a + WordList
{
    pub fn new(start: String, end: String, words: &'a WL) -> Self {
        WordSearch {
            start: start,
            end: end,
            words: words
        }
    }

    pub fn adjacent_words(&self, target: &str, threads: usize) -> Vec<String> {
        let alphabet_len = ALPHABET.chars().count();
        let target_len = target.chars().count();
        let capacity = alphabet_len * target_len * 3 + alphabet_len + target_len;
        let mut out = Vec::<String>::with_capacity(capacity);

        let mut insertion = String::with_capacity(target_len + 1);
        for a in ALPHABET.chars() {
            insertion.push(a);
            for c in target.chars() {
                insertion.push(c);
            };
            self.insert_if_new(&mut out, insertion.clone(), target);
            insertion.clear();
        }

        let mut local_vecs = Vec::<Vec<String>>::with_capacity(threads);
        for _ in 0..threads {
            local_vecs.push(Vec::<String>::with_capacity(alphabet_len * 2 + 2));
        }

        let mut pool = Pool::new(threads);

        pool.for_(local_vecs.iter_mut(), |mut local_vec| {
            // temporary string that we use and clear repeatedly
            let mut t = String::with_capacity(target_len + 1);
            // for each character in the target
            for i in 0..target_len {
                // Remove char i
                for (pos, c) in target.char_indices() {
                    if pos != i { t.push(c); }
                }
                self.insert_if_new_and_clear(&mut local_vec, &mut t, target);

                // Swap char i with char i + 1
                let mut last_pos = 0;
                for (pos, c) in target.char_indices() {
                    if pos > 0 && last_pos == i {
                        t.insert(i, c);
                    } else {
                        t.push(c);
                    }
                    last_pos = pos;
                }
                self.insert_if_new_and_clear(&mut local_vec, &mut t, target);

                // Edit
                for a in ALPHABET.chars() {
                    // Create a new string with the one letter changed
                    for (pos, c) in target.char_indices() {
                        if pos == i {
                            t.push(a);
                        } else {
                            t.push(c);
                        }
                    }

                    self.insert_if_new_and_clear(&mut local_vec, &mut t, target);
                }

                // Insert
                for a in ALPHABET.chars() {
                    for (pos, c) in target.char_indices() {
                        t.push(c);
                        if pos == i { t.push(a) }
                    };

                    self.insert_if_new_and_clear(&mut local_vec, &mut t, target);
                }
            }
        });
        for ref mut vec in local_vecs { out.append(vec); }
        out
    }

    fn insert_if_new<'b>(&'b self, out: &'b mut Vec<String>, word: String, target: &str) {
        if word == *target { return; }
        let mut all_words: Vec<&str> = Vec::new();
        for w in word.split_whitespace() {
            all_words.push(w.trim());
        }
        if all_words.iter().all(|w| self.words.find_word(w)) {
            out.push(all_words.join(" ").trim().to_string());
        }
    }
    
    fn insert_if_new_and_clear<'b>(&'b self, out: &'b mut Vec<String>, word: &'a mut String, target: &str) {
        self.insert_if_new(out, word.clone(), target);
        word.clear();
    }
}

impl<'a, WL> SearchProblem for WordSearch<'a, WL> 
where WL: 'a + WordList
{
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
        let adj: Vec<String> = self.adjacent_words(&node, 4);
        Box::new(adj.into_iter().map(|w| (w, 1i32)))
    }
}

#[test]
fn test_path() {
    let inner_path = vec!["we", "not"];

    let words = EnglishWordList::new(Path::new(&WORDS_PATH)).unwrap();

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
    let adjacent = words.adjacent_words(&w.to_owned(), 1);
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
    let adjacent = words.adjacent_words(&w.to_owned(), 1);
    println!("{:?}", adjacent);
    // Ensure insertion works
    assert!(adjacent.iter().find(|a| *a == &"tree house".to_owned()).is_some());
}

#[test]
fn test_insert_if_new() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let mut out = Vec::<String>::new();
    let w = "tree house".to_owned();
    words.insert_if_new(&mut out, w, &"treehouse".to_owned()); println!("{:?}", &out);
    assert!(out.iter().find(|a| *a == &"tree house".to_owned()).is_some());
}

#[test]
fn test_deletes_space() {
    let words = WordList::new(Path::new(&WORDS_PATH)).unwrap();
    let w = "re member";
    let adjacent = words.adjacent_words(&w.to_owned(), 1);
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

