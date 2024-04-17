use core::panic;
use std::collections::HashSet;
use std::path::Path;
use std::fs::{self, read_to_string, File};
use std::{env, io, process};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;

#[derive(PartialEq)]
enum JapaneseWordType {
    Hiragana,
    Katakana,
    Neither,
}


struct JapaneseWordParser {
    word_type: JapaneseWordType,
    word: String,
    changed: bool,
}

impl JapaneseWordParser {
    fn new() -> Self {
        JapaneseWordParser {
            word_type: JapaneseWordType::Neither,
            word: String::new(),
            changed: false,
        }
    }

    fn add_to_word(&mut self, s: char ) -> Option<String> {
        if s >= '\u{3040}' && s <= '\u{309f}' {
            if self.word_type != JapaneseWordType::Hiragana {
                return self.switch_word_type(JapaneseWordType::Hiragana, s)
            } 
            self.word.push(s);
            None
        } else if s >= '\u{30A0}' && s <= '\u{30FF}' {
            if self.word_type != JapaneseWordType::Katakana {
                return self.switch_word_type(JapaneseWordType::Katakana, s)
            }
            self.word.push(s);
            None
        } else {
            match self.switch_word_type(JapaneseWordType::Neither, ' ') {
                None => None,
                Some(s) => {
                    return Some(s)
                }
            }
        }
    }

    fn switch_word_type(&mut self, t: JapaneseWordType, c: char) -> Option<String> {
        // Add logic, that if the type was neither, and it's switching to 
        // Hiragana or katakana, push the char to the self.word
        if self.word_type != t {
            self.changed = true;
            self.word.push(c);
            None
        } else if t == JapaneseWordType::Neither {
            self.word_type = t;
            let final_word = &self.word;
            if final_word == "" {
                None
            } else {
                Some(final_word.to_string())
            }
        } else {
            self.changed = false;
            self.word.push(c);
            None
        }
    }
}


fn main() {
    let local_args: Vec<String> = env::args().collect();
    let arg_count = local_args.len();

    if arg_count == 1 {
        eprintln!("No file given");
        process::exit(1);
    }
    if arg_count > 2 {
        eprintln!("To many local_args!");
        process::exit(1);
    }

    
    let save_file = "data/word_list.txt";

    if !Path::new(save_file).exists() {
        let _dir = fs::create_dir("data");
        let _file = File::create(save_file)
            .expect("Could not create file");
    }


    let arg_file = fs::read_to_string(&local_args[1])
        .expect("Could not read file");


    let mut p = JapaneseWordParser::new();
    let mut words: Vec<String> = Vec::new();

    for ch in arg_file.chars() {
        match p.add_to_word(ch) {
            None => (),
            Some(s) => {
                words.push(s);
                p.word.clear();
            }
        }
    }

    words.sort();

    // to learn
    // What the crap is a hash set and how does it work?
    let mut seen = HashSet::new();

    words.retain(|word|
                 seen.insert(word.clone()));

    let file = File::open(&save_file).expect("Could not open file");
    let mut reader = BufReader::new(file);
    let mut file_words: Vec<String> = Vec::new();


    for lines in reader.lines() {
        match lines {
            Ok(v) => file_words.push(v),
            _ => panic!("Could not get line from bufreader"),
        }
    }


    file_words.append(&mut words);

    let mut seen = HashSet::new();
    file_words.retain(|word|
                      seen.insert(word.clone()));

    file_words.sort();

    let file = File::create(&save_file).expect("Could not open file");
    let mut file_writer = BufWriter::new(file);

    for word in file_words {
        writeln!(file_writer, "{}", word).expect("Failed to write to file");
    }


    





    // todo
    // I got all the words, and they are in words
    // Now, I need to:
    // 1) load words already in file 
    // 2) add and 
    // 3) sort the combined list of words
    // 4) Write to file
}

