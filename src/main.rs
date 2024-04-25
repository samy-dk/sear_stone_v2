// todo:
// - [ ] Implement ability to process flags to:
//      - [ ] Print out Random words
//      - [ ] List words
// - [ ] Japanese words can have the same pronouciation, but different 
//          meanings. Add functionality to store words and meanings, with 
//          labels if they are verbs, nouns etc.
// - [ ] Print out Random words of a specific type
//
//
// todo:
// - [ ] Add processing args
//      - [ ] This should happen in main. Use a struct to contain the args
//              that aren't flags, and an enum for the different flags

use std::{env, process};





fn main() {
    let local_args: Vec<String> = env::args().collect();
    let arg_count = local_args.len();
    let mut data = ss_data::SSData::new();

    if arg_count == 1 {
        eprintln!("Not enough args. Try ss -h or --help for help");
        process::exit(1);
    } 
    for a in local_args {
        match a.as_str().trim() {
            "-h" => data.flags = Some(ss_data::Flags::Help),
            "--help" => data.flags = Some(ss_data::Flags::Help),
            "-pa" => data.flags = Some(ss_data::Flags::PrintAll),
            "--print-all" => data.flags = Some(ss_data::Flags::PrintAll),
            "-pr" => data.flags = Some(ss_data::Flags::PrintRandom),
            "--print-random" => data.flags = Some(ss_data::Flags::PrintRandom),
            _ => println!("Arg: {}", a),
        }
    }

    match data.flags {
        Some(ss_data::Flags::Help) => processes::print_help(),
        Some(ss_data::Flags::PrintAll) => processes::print_all(),
        Some(ss_data::Flags::PrintRandom) => processes::print_random(),
        None => processes::process_files(),
    }

}


mod ss_data {
    pub enum Flags {
        Help,
        PrintAll,
        PrintRandom,
    }

    pub struct SSData {
        pub flags: Option<Flags>,
        pub file_args: Option<Vec<String>>,
    }

    impl SSData {
        pub fn new() -> SSData {
            let d = SSData {
                flags: None,
                file_args: None,
            };
            d
        }
    }
}


mod processes {
    use crate::structures::JapaneseWordParser;
    use core::panic;
    use std::collections::HashSet;
    use std::path::Path;
    use std::fs::{self, File};
    use std::{env, process};
    use std::io::{BufReader, BufWriter};
    use std::io::prelude::*;


    // This is the main funciton. This will take a file and read the words,
    // combine with the old list, remove duplicates, sort and then store the
    // words!
    pub fn process_files() {
        // process the args
        let local_args: Vec<String> = env::args().collect();
        let arg_count = local_args.len();

        if arg_count == 1 {
            eprintln!("No file given");
            process::exit(1);
        }


        // make sure the word_list.txt file is there
        let save_file = "data/word_list.txt";

        if !Path::new(save_file).exists() {
            let _dir = fs::create_dir("data");
            let _file = File::create(save_file)
                .expect("Could not create file");
        }


        // read the file from the args


        // Process chars, one-by-one, from multiple files.
        // 
        // problem
        // current implementation only accounts for there being only files as
        // args, no flags. Could replace the bool with a counter that subtacts
        // 1 from the number of flags, and once it gets to zero, simply 
        // processes the rest of the args as files.
        let mut p = JapaneseWordParser::new();
        let mut words: Vec<String> = Vec::new();

        let mut c = false;
        for a in local_args {
            if c == false {
                c = true;
            } else {
                let arg_file = fs::read_to_string(&a)
                    .expect("Could not read file");
                for ch in arg_file.chars() {
                    match p.add_to_word(ch) {
                        None => (),
                        Some(s) => {
                            words.push(s);
                            p.word.clear();
                        }
                    }
                }

            }
        }
        words.sort();


        // to learn
        // What the crap is a hash set and how does it work?
        //
        // this removes duplicates
        let mut seen = HashSet::new();

        words.retain(|word|
                     seen.insert(word.clone()));


        // get words that were already in the file
        let file = File::open(&save_file).expect("Could not open file");
        let reader = BufReader::new(file);
        let mut file_words: Vec<String> = Vec::new();

        for lines in reader.lines() {
            match lines {
                Ok(v) => file_words.push(v),
                _ => panic!("Could not get line from bufreader"),
            }
        }


        // add the words together, remove duplicates and sort
        file_words.append(&mut words);

        let mut seen = HashSet::new();
        file_words.retain(|word|
                          seen.insert(word.clone()));

        file_words.sort();


        // save the new list of words to the file!
        let file = File::create(&save_file).expect("Could not open file");
        let mut file_writer = BufWriter::new(file);

        for word in file_words {
            writeln!(file_writer, "{}", word).expect("Failed to write to file");
        }
    }

    pub fn print_help() {
        println!("Welcome to sear_stone! This program pulls Japanese words out");
        println!("of files to aid in study.");
        println!("");
        println!("Here is how to use it:");
        println!("1) If no flags are passed, all args are assumed to be text ");
        println!("files that contains Japanese words and will be proccessed.");
        println!("These words will be added to a file called words_list.txt");
        println!("");
        println!("2) -h or --help will print this menu.");
        println!("");
        println!("3) -pa or --print-all will print all the words currently in ");
        println!("words_list.txt");
        println!("");
        println!("4) -pr or --print-random will print 10 random words from ");
        println!("words_list.txt");
        println!("");
    }

    pub fn print_all() {
        let save_file = "data/word_list.txt";

        if !Path::new(save_file).exists() {
            let _dir = fs::create_dir("data");
            let _file = File::create(save_file)
                .expect("Could not create file");
        }

        let file = File::open(&save_file).expect("Could not open file");
        let reader = BufReader::new(file);

        for l in reader.lines() {
            println!("{:?}", l);
            match l {
                Ok(w) => println!("{}", w),
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    pub fn print_random() {
        println!("I'm getting called but I need to be implemented");
    }
}

mod structures {
    // JapaneseWordType is used to help the program know when what it's 
    // reading is Hiragana, Katakana, or neither. It's important, because 
    // words must be of the same type. Once the type changes, it must be a new
    // word, or not a word at all.
    #[derive(PartialEq)]
    pub enum JapaneseWordType {
        Hiragana,
        Katakana,
        Neither,
    }


    // JapaneseWordParser is used to parser together japanese words. It's 
    // meant to be used by feeding it a stream of chars. Based on the input
    // of chars, it will either 1) add the char to the word, 2) do nothing
    // if it is not a word, or 3) return a word if the chars change type.
    pub struct JapaneseWordParser {
        pub word_type: JapaneseWordType,
        pub word: String,
        pub changed: bool,
    }


    impl JapaneseWordParser {
        pub fn new() -> Self {
            JapaneseWordParser {
                word_type: JapaneseWordType::Neither,
                word: String::new(),
                changed: false,
            }
        }


        // main parser. Takes in a char, returns nothing if it's the same type,
        // adds a char to the current word being processes 
        // (JapaneseWordParser.word) or it returns a new word if the type has
        // changed.
        pub fn add_to_word(&mut self, s: char ) -> Option<String> {
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


        // used by JapaneseWordParser.add_to_word  If the type changes, it 
        // must do different things based on the previous type. 
        fn switch_word_type(&mut self, t: JapaneseWordType, c: char) -> Option<String> {
            // The inital if covers the case that the first char is of a certain type. 
            // without it, you would get all the words back w/o the first
            // character, which is pointless
            //
            // first if else covers if it encounters a non japanese char, if 
            // there is something to return, return it.
            // todo:
            // - [ ] Find out what happens when a katakana is next to a 
            //          hiragana char, or vic-versa. Does it end up returning
            //          a mixed hiragana and katakana word?
            //  
            //  the else statment covers if nothing has changed, just add the
            //  char to the word.
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
}
