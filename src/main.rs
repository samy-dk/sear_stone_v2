// todo:
//-----------------------------------------------------------------------------
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
            "-s" => data.flags = Some(ss_data::Flags::AddMeaning),
            "--set-meaning" => data.flags = Some(ss_data::Flags::AddMeaning),
            "-aw" => data.flags = Some(ss_data::Flags::AddWord),
            "--add-word" => data.flags = Some(ss_data::Flags::AddWord),
            "-rw" => data.flags = Some(ss_data::Flags::RemoveWord),
            "--remove-word" => data.flags = Some(ss_data::Flags::RemoveWord),
            "-t" => data.flags = Some(ss_data::Flags::Test),
            "--test" => data.flags = Some(ss_data::Flags::Test),
            _ => println!("Arg: {}", a),
        }
    }

    match data.flags {
        Some(ss_data::Flags::Help) => processes::print_help(),
        Some(ss_data::Flags::PrintAll) => processes::print_all(),
        Some(ss_data::Flags::PrintRandom) => processes::print_random(),
        Some(ss_data::Flags::AddMeaning) => processes::set_meaning(),
        Some(ss_data::Flags::AddWord) => processes::add_word(),
        Some(ss_data::Flags::RemoveWord) => processes::remove_word(),
        Some(ss_data::Flags::Test) => processes::test(),
        None => processes::process_files().expect("Processing Failed"),
    }

}


mod ss_data {
    pub enum Flags {
        Help,
        PrintAll,
        PrintRandom,
        AddMeaning,
        AddWord,
        RemoveWord,
        Test,
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
    use core;
    use std::process::exit;
    use std::{collections, path, fs, env, process, io};
    use std:: error::Error;
    use std::io::prelude::*;
    use rand::Rng;
    use crate::structures::{JPWord, JapaneseWordParser, WordType};


    const J_SAVE_FILE: &str = "data/word_list.json";


    // This is the main funciton. This will take a file and read the words,
    // combine with the old list, remove duplicates, sort and then store the
    // words!
    pub fn process_files() -> Result<(), Box<dyn Error>> {
        // process the args
        let local_args: Vec<String> = env::args().collect();
        let arg_count = local_args.len();

        if arg_count == 1 {
            eprintln!("No file given");
            process::exit(1);
        }


        // make sure the word_list.txt file is there
        if !path::Path::new(J_SAVE_FILE).exists() {
            let _dir = fs::create_dir("data");
            let _file = fs::File::create(J_SAVE_FILE)
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
        let mut words: Vec<JPWord> = Vec::new();

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
                            words.push(JPWord::simple_new(s));
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
        let mut seen: collections::HashSet<JPWord> = collections::HashSet::new();


        words.retain(|word|
                     seen.insert(word.clone()));


        // get words that were already in the file
        //
        let f_words = fs::read_to_string(J_SAVE_FILE).expect("Could not read file");
                            
        // d_f_words means deserialized_file_words
        let mut d_f_words: Vec<JPWord> = if f_words.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&f_words).expect("Could not parse json from file")
        };


        // add the words together, remove duplicates and sort
        d_f_words.append(&mut words);

        let mut seen = collections::HashSet::new();
        d_f_words.retain(|word|
                          seen.insert(word.clone()));

        d_f_words.sort();


        // save the new list of words to the file!

        let stringified =  serde_json::to_string(&d_f_words).expect("Could not parse into JSON before writing");
        fs::write(J_SAVE_FILE, &stringified)?;
        Ok(())

    }


    // todo - needs updating
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

        if !path::Path::new(J_SAVE_FILE).exists() {
            let _dir = fs::create_dir("data");
            let _file = fs::File::create(J_SAVE_FILE)
                .expect("Could not create file");
        }

        let f_words = fs::read_to_string(J_SAVE_FILE).expect("Could not read file");

        let d_f_words: Vec<JPWord> = if f_words.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&f_words).expect("Could not parse json from file")
        };

        for w in d_f_words {
            println!("{}", w.word);
        }
    }

    pub fn print_random() {
        if !path::Path::new(J_SAVE_FILE).exists() {
            let _dir = fs::create_dir("data");
            let _file = fs::File::create(J_SAVE_FILE)
                .expect("Could not create file");
        }

        let f_words = fs::read_to_string(J_SAVE_FILE).expect("Could not read file");

        let d_f_words: Vec<JPWord> = if f_words.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&f_words).expect("Could not parse json from file")
        };

        // todo
        // implement check to make sure that the same two words are printed

        let limit = 10;
        let mut rng = rand::thread_rng();
        let vec_len = d_f_words.len();
        let ran_num: Vec<usize> = (0..limit).map(|_|
                                    rng.gen_range(0..=vec_len-1)).collect();

        for n in ran_num {
            println!("{}", d_f_words[n].word);
        }

    }

    pub fn set_meaning() -> () {
        // Read current list of words, in word_list.json
        if !path::Path::new(J_SAVE_FILE).exists() {
            let _dir = fs::create_dir("data");
            let _file = fs::File::create(J_SAVE_FILE)
                .expect("Could not create file");
        }

        let f_words = fs::read_to_string(J_SAVE_FILE).expect("Could not read file");

        let mut d_f_words: Vec<JPWord> = if f_words.trim().is_empty() {
            eprintln!("File is empty! Can't define any words...");
            process::exit(1);
        } else {
            serde_json::from_str(&f_words).expect("Could not parse json from file")
        };

        // prompt user for the word they would like to define
        let mut u_prompt: String = String::new();
        println!("Please enter the word you'd like to define: ");

        io::stdin().read_line(&mut u_prompt).expect("Could not read input");

        if u_prompt.trim().is_empty() {
            eprintln!("Nothing entered! Exiting program now");
            process::exit(1);
        }

        // find that word
        let index = d_f_words.iter().position(|item| item.word == u_prompt.trim());

        // if I can find the word, grab it's index
        let index = match index {
            Some(i) => i,
            None => {
                // if I can't find the word, let the user know and end
                eprintln!("Could not find that word! Exiting program now");
                process::exit(1);
            }
        };
        println!("I found the word! Let's add some definition to it!");

        // prompt the user to add the following:
        // word type, this will list the word types and ask the user to 
        //     enter a number that corresponds with that type
        // definition, user will type out the definition
        let mut u_prompt = String::new();

        println!("What type of word is it?");
        println!("Please enter the corresponding number, or 0 to skip");
        println!("1  -> Noun");
        println!("2  -> Pronoun");
        println!("3  -> Verb");
        println!("4  -> Adjective");
        println!("5  -> Adverb");
        println!("6  -> Preposition");
        println!("7  -> Conjuntion");
        println!("8  -> Interjunction");
        println!("9  -> Article");
        println!("10 -> Quantifier");
        println!("11 -> Auxiliary");
        println!("12 -> Phrase");
        io::stdin()
            .read_line(&mut u_prompt)
            .expect("Could not read input from user");
        let u_prompt: u32 = match u_prompt.trim().parse() {
            Ok(num) => num,
            Err(e) => panic!("Could not parse number: {}", e),
        };

        d_f_words[index].word_type = match u_prompt {
            0 => None,
            1 => Some(WordType::Noun),
            2 => Some(WordType:: Pronoun),
            3 => Some(WordType::Verb),
            4 => Some(WordType::Adjective),
            5 => Some(WordType::Adverb),
            6 => Some(WordType::Preposition),
            7 => Some(WordType::Conjunction),
            8 => Some(WordType::Interjection),
            9 => Some(WordType::Article),
            10 => Some(WordType::Quantifier),
            11 => Some(WordType::Auxiliary),
            _ => {
                eprintln!("Invalid number. Skipping to next part");
                None
            }
        };

        println!("Alright, what does this word mean?");
        println!("Please, do not hit enter or enter a newline char in your response");

        let mut u_prompt = String::new();
        io::stdin()
            .read_line(&mut u_prompt)
            .expect("Could not read input from user");

        d_f_words[index].definition = Some(u_prompt);

        // save the new vec to the file
        let stringified = serde_json::to_string(&d_f_words).expect("Could not parse into JSON before writing");
        fs::write(J_SAVE_FILE, &stringified)
            .expect("Could not write to file");
    }


    pub fn add_word() -> () {
        if !path::Path::new(J_SAVE_FILE).exists() {
            let _dir = fs::create_dir("data");
            let _file = fs::File::create(J_SAVE_FILE)
                .expect("Could not create file");
        }

        let f_words = fs::read_to_string(J_SAVE_FILE).expect("Could not read file");

        let mut d_f_words: Vec<JPWord> = if f_words.trim().is_empty() {
            eprintln!("File is empty! Can't define any words...");
            process::exit(1);
        } else {
            serde_json::from_str(&f_words).expect("Could not parse json from file")
        };

        let mut input = String::new();
        println!("What word would you like to add?");
        io::stdin().read_line(&mut input).expect("Could not read input");

        // Check if word is in list
        for w in &d_f_words {
            if w.word == String::from(input.trim()) {
                eprintln!("Sorry, that word is already in the list!!!");
                process::exit(0);
            }
        }

        d_f_words.push(JPWord::simple_new(String::from(input.trim())));
        let word_index = d_f_words.len() - 1;

        let mut u_prompt = String::new();

        println!("What type of word is it?");
        println!("Please enter the corresponding number, or 0 to skip");
        println!("1  -> Noun");
        println!("2  -> Pronoun");
        println!("3  -> Verb");
        println!("4  -> Adjective");
        println!("5  -> Adverb");
        println!("6  -> Preposition");
        println!("7  -> Conjuntion");
        println!("8  -> Interjunction");
        println!("9  -> Article");
        println!("10 -> Quantifier");
        println!("11 -> Auxiliary");
        println!("12 -> Phrase");
        io::stdin()
            .read_line(&mut u_prompt)
            .expect("Could not read input from user");
        let u_prompt: u32 = match u_prompt.trim().parse() {
            Ok(num) => num,
            Err(e) => panic!("Could not parse number: {}", e),
        };

        d_f_words[word_index].word_type = match u_prompt {
            0 => None,
            1 => Some(WordType::Noun),
            2 => Some(WordType:: Pronoun),
            3 => Some(WordType::Verb),
            4 => Some(WordType::Adjective),
            5 => Some(WordType::Adverb),
            6 => Some(WordType::Preposition),
            7 => Some(WordType::Conjunction),
            8 => Some(WordType::Interjection),
            9 => Some(WordType::Article),
            10 => Some(WordType::Quantifier),
            11 => Some(WordType::Auxiliary),
            _ => {
                eprintln!("Invalid number. Skipping to next part");
                None
            }
        };

        println!("Alright, what does this word mean?");
        println!("Please, do not hit enter or enter a newline char in your response");

        let mut u_prompt = String::new();
        io::stdin()
            .read_line(&mut u_prompt)
            .expect("Could not read input from user");

        d_f_words[word_index].definition = Some(u_prompt);

        // save the new vec to the file
        let stringified = serde_json::to_string(&d_f_words).expect("Could not parse into JSON before writing");
        fs::write(J_SAVE_FILE, &stringified)
            .expect("Could not write to file");
    }

    pub fn remove_word() -> () {
        if !path::Path::new(J_SAVE_FILE).exists() {
            let _dir = fs::create_dir("data");
            let _file = fs::File::create(J_SAVE_FILE)
                .expect("Could not create file");
        }

        let f_words = fs::read_to_string(J_SAVE_FILE).expect("Could not read file");

        let mut d_f_words: Vec<JPWord> = if f_words.trim().is_empty() {
            eprintln!("File is empty! Can't define any words...");
            process::exit(1);
        } else {
            serde_json::from_str(&f_words).expect("Could not parse json from file")
        };

        let mut input = String::new();
        println!("What word would you like to remove?");
        io::stdin().read_line(&mut input).expect("Could not read input");

        // Check if word is in list
        let mut exist = false;
        for w in &d_f_words {
            if w.word == String::from(input.trim()) {
                exist = true;
                break;
            }
        }

        if !exist {
            eprintln!("Sorry, that word does not exist in the list");
            process::exit(0);
        }

        d_f_words.retain(|x| x.word != input.trim());
        println!("Succesfully removed the word!");

        // save the new vec to the file
        let stringified = serde_json::to_string(&d_f_words).expect("Could not parse into JSON before writing");
        fs::write(J_SAVE_FILE, &stringified)
            .expect("Could not write to file");
    }

    pub fn test() -> () {
        if !path::Path::new(J_SAVE_FILE).exists() {
            let _dir = fs::create_dir("data");
            let _file = fs::File::create(J_SAVE_FILE)
                .expect("Could not create file");
        }

        let f_words = fs::read_to_string(J_SAVE_FILE).expect("Could not read file");

        let d_f_words: Vec<JPWord> = if f_words.trim().is_empty() {
            eprintln!("File is empty! Can't define any words...");
            process::exit(1);
        } else {
            serde_json::from_str(&f_words).expect("Could not parse json from file")
        };

        let mut rng = rand::thread_rng();
        let r_num = rng.gen_range(0..=&d_f_words.len()-1);

        println!("Here is a test!");
        println!("What is {} ?", d_f_words[r_num].word);

        let mut _buffer = String::new();
        let _input = io::stdin().read_line(&mut _buffer);

        let def_option = &d_f_words[r_num]
            .definition;
        let def = match def_option {
            Some(v) => String::from(v.trim()),
            None => String::from("No Definition"),
        };
        println!("\nThe answer is:\n{}", def);
    }

}


mod structures {
    use serde::{Serialize, Deserialize};
    use core::fmt;
    use std::cmp::Ordering;


    
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
    pub enum WordType {
        Noun,
        Pronoun,
        Verb,
        Adjective,
        Adverb,
        Preposition,
        Conjunction,
        Interjection,
        Article,
        Quantifier,
        Auxiliary,
        Phrase
    }


    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
    pub struct JPWord {
        pub word: String,
        pub word_type: Option<WordType>,
        pub definition: Option<String>,
    }


    impl JPWord {
        pub fn simple_new (w: String) -> JPWord {
            let _word = JPWord {
                word: w,
                word_type: None,
                definition: None,
            };
            _word
        }

    }

    impl PartialOrd for JPWord {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
        
    impl Ord for JPWord {
        fn cmp(&self, other: &Self) -> Ordering {
            self.word.cmp(&other.word)
        }
    }

    impl fmt::Display for JPWord {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.word)
        }
    }


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
            //          hiragana char, or vis-versa. Does it end up returning
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
