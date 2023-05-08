use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::Instant;
use std::collections::HashMap;
use rand::prelude::SliceRandom;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

static FILE_PATH_EN: &str = "data/words.txt";
//static FILE_PATH_TEST: &str = "data/test.txt";

pub fn start_game() {
    Menu::new().start();
}

enum MenuAction {
    StartGame,
    ExitGame,
}

struct Menu {
    word_length: i16,
    language: String,
}

impl Menu {
    fn new() -> Menu {
        Menu { word_length: 5, language: "en".to_string() }
    }

    fn start(&mut self) {
        println!("Welcome to Wordle solveur");
        loop {
            match self.menu() {
                MenuAction::StartGame => {
                    if Game::new(self.word_length, self.language.clone()).start() {
                        println!("Returning to the main menu.");
                    } else {
                        println!("Exiting the game.");
                        break;
                    }
                }
                MenuAction::ExitGame => {
                    println!("Goodbye!");
                    break;
                }
            }
        }
    }

    fn menu(&mut self) -> MenuAction {
        println!("Let's play with {} letter words in English.", self.word_length);
        println!("Press Enter to start the game.");
        println!("1. Modify word length");
        println!("2. Modify language");
        println!("3. Exit the game");
        print!("=> ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "" => MenuAction::StartGame,
            "1" => {
                println!("Modifying word length");
                self.word_length = Menu::choose_word_length();
                self.menu()
            }
            "2" => {
                println!("Modifying language");
                self.menu()
            }
            "3" => MenuAction::ExitGame,
            _ => {
                println!("Invalid option");
                self.menu()
            }
        }
    }

    fn choose_word_length() -> i16 {
        println!("Please enter the desired word length for the game (between 2 and 20):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input.parse::<i16>() {
            Ok(word_length) => {
                if word_length < 2 || word_length > 20 {
                    println!("Invalid input. Please enter a number between 2 and 20.");
                    Menu::choose_word_length()
                } else {
                    word_length
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number between 2 and 20.");
                Menu::choose_word_length()
            }
        }
    }
}

struct Game {
    size: i16,
    language : String,
    possibilities: Vec<Vec<i8>>,
    all_words: Vec<String>,
    words: Vec<String>,
}

impl Game {
    fn new(size: i16, language: String) -> Game {
        let words = Self::import_words(&language.to_string(), size);
        let possibilities = Game::generate_possibilities(size as usize);
        let all_words = words.clone();

        Game { size, language, possibilities, all_words, words }
    }

    fn generate_possibilities(n: usize) -> Vec<Vec<i8>> {
        let possibilities = vec![0, 1, 2];
        if n == 0 {
            return vec![vec![]];
        }
        let mut result = Vec::new();
        for combination in Game::generate_possibilities(n - 1).iter() {
            for possibility in possibilities.iter() {
                let mut new_combination = combination.clone();
                new_combination.push(*possibility as i8);
                result.push(new_combination);
            }
        }
        result
    }

    fn import_words(language: &String, size: i16) -> Vec<String> {
        let path: &Path;
        if language == "en" {
            path = Path::new(FILE_PATH_EN);
        } else {
            path = Path::new(FILE_PATH_EN);
        }
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line.expect("Failed to read line").to_uppercase())
            .filter(|word| word.len() == size as usize)
            .collect()
    }

    fn start(&mut self) -> bool {
        self.start_first();
        self.continue_game()
    }

    fn continue_game(&mut self) -> bool {
        let start_time = Instant::now();
        let default_word = String::from("UNKNOWN");
        let default_esperance = 0;
        let word = self
            .find_best()
            .unwrap_or_else(|| {
                println!("Aucun mot trouvé");
                (&default_word, default_esperance)
            });

        let elapsed_time = start_time.elapsed();
        print!("{}  --> {}  time {:?}\n", word.0, word.1, elapsed_time);
        let reply = self.input_sequence();
        let word_color = Game::display_colored_text(word.0, &reply);
        let words_len_before = self.words.len();
        self.words = self.eliminate(word.0, reply);
        let words_len_after = self.words.len();
        if words_len_after > 1 {
            println!("{}  --> {} {:?} {}", word_color, words_len_before - words_len_after, self.words, words_len_after);
            self.continue_game()
        } else if words_len_after == 1 {
            println!("The word is {}", self.words[0]);
            true // Retourner au menu principal
        } else {
            println!("Something went wrong");
            false // Terminer le jeu
        }
    }

    fn start_first(&mut self) {
        let first_word = self.ouverture();
        println!("Enter a sequence of {} numbers (only 0, 1, or 2): ", self.size);
        println!("{}  --> {}", first_word.0,first_word.1);

        let first_reply = self.input_sequence();
        let word_color = Game::display_colored_text(&first_word.0, &first_reply);
        let words_len_before = self.words.len();
        self.words = self.eliminate(&first_word.0, first_reply);
        println!("{}  --> {} words eliminate ", word_color, words_len_before - self.words.len());
    }

    fn ouverture(&self) -> (String, String) {
        let mut best_ouverture = HashMap::<usize, (String, String)>::new();

        if self.language == "en" {
            best_ouverture.insert(2, ("HO".to_string(), "with an expected value of 27.5".to_string()));
            best_ouverture.insert(3, ("EAT".to_string(), "with an expected value of 462.3".to_string()));
            best_ouverture.insert(4, ("SALE".to_string(), "with an expected value of 2146.6".to_string()));
            best_ouverture.insert(5, ("TARES".to_string(), "with an expected value of 4175.6".to_string()));
            best_ouverture.insert(6, ("SAILER".to_string(), "with an expected value of 6877.7".to_string()));
            best_ouverture.insert(7, ("SALTIER".to_string(), "with an expected value of 9173.5".to_string()));
            best_ouverture.insert(8, ("NOTARIES".to_string(), "with an expected value of 9380.1".to_string()));
        }

        if self.language == "fr" {
            best_ouverture.insert(2, ("eu".to_string(), "with an expected value of 46.691".to_string()));
            best_ouverture.insert(3, ("aie".to_string(), "with an expected value of 361.133".to_string()));
            best_ouverture.insert(4, ("raie".to_string(), "with an expected value of 1 707.937".to_string()));
            best_ouverture.insert(5, ("raies".to_string(), "with an expected value of 5 784.177".to_string()));
            best_ouverture.insert(6, ("taries".to_string(), "with an expected value of 13 801.754".to_string()));
            best_ouverture.insert(7, ("ratines".to_string(), "with an expected value of 25 368.590".to_string()));
            best_ouverture.insert(8, ("rancites".to_string(), "with an expected value of 38 023.956".to_string()));
        }

        match best_ouverture.get(&(self.size as usize)) {
            Some((word, info)) => (word.clone(), info.clone()),
            None => (self.words.choose(&mut rand::thread_rng()).unwrap().to_string(), "".to_string()),
        }
    }


    fn input_sequence(&self) -> Vec<i8> {
        let mut result = vec![];
        loop {
            let mut sequence = String::new();
            io::stdin().read_line(&mut sequence).unwrap();
            let sequence = sequence.trim();
            if sequence.len() != self.size as usize {
                println!("\x1B[31mInvalid input, the sequence must be of length {}\x1B[0m", self.size);
                continue;
            }
            let mut invalid_input = false;
            for i in sequence.chars() {
                match i {
                    '2' => {
                        result.push(2);
                    }
                    '1' => {
                        result.push(1);
                    }
                    '0' => {
                        result.push(0);
                    }
                    _ => {
                        println!("\x1B[31mInvalid input, only 0, 1, or 2 are allowed\x1B[0m");
                        result.clear();
                        invalid_input = true;
                        break;
                    }
                }
            }
            if !invalid_input {
                break;
            }
        }
        result
    }

    fn display_colored_text(text: &String, colors: &Vec<i8>) -> String {
        let mut output = String::new();
        for (i, c) in text.chars().enumerate() {
            match colors[i] {
                0 => output.push_str(&format!("{}", c)),
                1 => output.push_str(&format!("\x1b[33m{}\x1b[0m", c)),
                2 => output.push_str(&format!("\x1b[32m{}\x1b[0m", c)),
                _ => output.push(c),
            }
        }
        output
    }

    fn find_best(&self) -> Option<(&String, i32)> {
        if self.words.is_empty() {
            return None;
        }

        let esperances: Arc<Mutex<Vec<(i32, &String)>>> = Arc::new(Mutex::new(Vec::new()));
        let progress: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
        let total_words = self.all_words.len();
        self.all_words.par_iter().enumerate().for_each(|(_i, word)| {
            let esperance = Self::compute_esperance(self, word);
            let mut esperances = esperances.lock().unwrap();
            esperances.push((esperance, word));
            let mut progress = progress.lock().unwrap();
            *progress += 1;
            Self::print_progress(*progress, total_words, 50);
        });

        print!("\r");
        for _ in 0..(50 + 10) {
            print!(" ");
        }
        print!("\r");

        let cloned_esperances = esperances.lock().unwrap().clone();
        let best_tuple = cloned_esperances
            .iter()
            .max_by_key(|(esperance, word)| {
                (*esperance, if self.words.contains(word) { 1 } else { 0 })
            })
            .map(|(esperance, word)| (*esperance, *word));

        best_tuple.map(|(esperance, word)| (word, esperance))
    }

    fn compute_esperance(&self, word: &String) -> i32 {
        let mut s_restant = 0;
        let mut esperance = 0;
        let len_words = self.words.len();
        //print!("{} | E=",len_words);
        for _possibles in self.possibilities.iter() {
            //print!("{}",Game::display_colored_text(word,&_possibles.to_vec()));
            let mots_restant = self.eliminate(word, _possibles.to_vec()).len();
            s_restant += mots_restant;
            esperance += mots_restant * (len_words - mots_restant);
            //print!("->{} + ",len_words - mots_restant);
        }
        let esperance = esperance / s_restant;
        //println!("\nE({})={} , {}=<{}\n",word,esperance,len_words,s_restant);
        esperance as i32
    }

    fn eliminate(&self, word: &String, reply: Vec<i8>) -> Vec<String> {
        let word_separate = Game::separate_strings(word, &reply);

        self.words.iter()
            .filter(|word|
                word_separate.2.iter()
                    .all(|(c, i)| word.chars().nth(*i) == Some(*c))
            )
            .filter(|word| {
                let word_chars: Vec<(usize, char)> = word.chars().enumerate().collect();
                word_separate.1.iter()
                    .all(|(c, i)| !word_chars.contains(&(*i, *c))) && word_separate.1.iter().all(|(c, _)| word.contains(*c))
            })
            .filter(|word| {
                word_separate.0.iter().all(|c| {
                    let c_count_in_word = word.chars().filter(|x| x == c).count();
                    let c_count_in_yellow = word_separate.1.iter().filter(|(x, _)| x == c).count();
                    let c_count_in_green = word_separate.2.iter().filter(|(x, _)| x == c).count();

                    if c_count_in_green + c_count_in_yellow > 0 {
                        c_count_in_word == c_count_in_green + c_count_in_yellow
                    } else {
                        !word.contains(*c)
                    }
                })
            })
            .cloned()
            .collect::<Vec<String>>()
    }

    fn separate_strings(s: &String, v: &Vec<i8>) -> (Vec<char>, Vec<(char, usize)>, Vec<(char, usize)>) {
        let mut l0 = vec![];
        let mut l1 = vec![];
        let mut l2 = vec![];
        for (i, c) in s.chars().enumerate() {
            match v[i] {
                0 => l0.push(c),
                1 => l1.push((c, i)),
                2 => l2.push((c, i)),
                _ => (),
            }
        }
        (l0, l1, l2)
    }

    fn print_progress(current: usize, total: usize, width: usize) {
        let progress = (current as f64) / (total as f64);
        let completed = (progress * (width as f64)).round() as usize;
        let incomplete = width - completed;
        print!("\r[");
        for _ in 0..completed {
            print!("#");
        }
        for _ in 0..incomplete {
            print!("-");
        }
        print!("] {:.2}%", progress * 100.0);
        io::stdout().flush().unwrap();
    }
}