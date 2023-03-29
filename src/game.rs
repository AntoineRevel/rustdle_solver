use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

static FILE_PATH_EN: &str = "data/words.txt";
static FILE_PATH_TEST: &str = "data/test.txt";

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
    possibilities: Vec<Vec<i8>>,
    all_words: Vec<String>,
    words: Vec<String>,
}

impl Game {
    fn new(size: i16, language: String) -> Game {
        let words = Self::import_words(&language.to_string(), size);
        let possibilities = Game::generate_possibilities(size as usize);
        let all_words = words.clone();

        Game { size, possibilities, all_words, words }
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
        let default_word = String::from("UNKNOWN");
        let default_esperance = 0;
        let word = self
            .find_best()
            .unwrap_or_else(|| {
                println!("Aucun mot trouvé");
                (&default_word, default_esperance)
            });

        print!("{}  --> {}\n", word.0, word.1);
        let reply = self.input_sequence();
        let word_color = Game::display_colored_text(word.0, &reply);
        let words_len_before = self.words.len();
        self.words = self.eliminate(word.0, reply);
        let words_len_after = self.words.len();
        if words_len_after > 1 {
            println!("{}  --> {} {:?} {}", word_color, words_len_before - words_len_after, self.words,words_len_after );
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
        let first_word = "TYRES".to_string();//self.words.choose(&mut rand::thread_rng()).unwrap();
        println!("Enter a sequence of {} numbers (only 0, 1, or 2): ", self.size);
        println!("{}  --> first random proposition", first_word);

        let first_reply = self.input_sequence();
        let word_color = Game::display_colored_text(&first_word, &first_reply);
        let words_len_before = self.words.len();
        self.words = self.eliminate(&first_word, first_reply);
        println!("{}  --> {} words eliminate", word_color, words_len_before - self.words.len());
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
        let mut max_esperance = 0;
        let mut best_word: &String = &self.words[0];
        let total_words = self.all_words.len();

        for (i, _word) in self.all_words.iter().enumerate() {
            let esperance = Self::compute_esperance(self, _word);
            if esperance > max_esperance || (esperance == max_esperance && self.words.contains(_word)) {
                max_esperance = esperance;
                best_word = _word;
            }

            Self::print_progress(i + 1, total_words, 50);
        }

        println!("\n");

        Some((best_word, max_esperance))
    }



    fn compute_esperance(&self, word: &String) -> i32 {
        let mut s_restant=0;
        let mut esperance = 0;
        let len_words = self.words.len();
        //print!("{} | E=",len_words);
        for _possibles in self.possibilities.iter() {
            //print!("{}",Game::display_colored_text(word,&_possibles.to_vec()));
            let mots_restant = self.eliminate(word, _possibles.to_vec()).len();
            s_restant+=mots_restant;
            esperance += mots_restant*(len_words - mots_restant);
            //print!("->{} + ",len_words - mots_restant);
        }
        let esperance = esperance / s_restant;
        //println!("\nE({})={} , {}=<{}\n",word,esperance,len_words,s_restant);
        esperance as i32

    }

    fn eliminate(&self, word: &String, reply: Vec<i8>) -> Vec<String> {
        let word_separate = Game::separate_strings(word, &reply);

        let mut words_reply = self.words.clone();
        //println!("{}",Game::display_colored_text(word,&reply));

        words_reply = words_reply.into_iter()
            .filter(|word|
                word_separate.2.iter()
                    .all(|(c, i)| word.chars().nth(*i) == Some(*c))
            )
            .filter(|word| {
                let word_chars: Vec<(char, usize)> = word.chars().enumerate().map(|(i, c)| (c, i)).collect();
                word_separate.1.iter()
                    .all(|(c, i)| !word_chars.contains(&(*c, *i))) && word_separate.1.iter().all(|(c, _)| word.contains(*c))
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
            .collect();
        words_reply
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

