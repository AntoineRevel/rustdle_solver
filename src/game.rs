use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

static FILE_PATH_EN : &str = "data/words.txt";

fn start_game() {

    let mut words =Words::new(4, "en");

    //Words::start(&mut words, reply);

    //print!("{:?}\n",words.words);
    //print!("{}",pos.len());

}


pub fn menu() {
    //default
    let mut word_length =5;

    println!("Welcome to wordle solveur");
    println!("1. Modify word length");
    println!("2. Modify language");
    println!("3. Exit the game\n");
    println!("Press Enter to start the game.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match input {
        "" => println!("Starting the game"),
        "1" => {println!("Modifying word length");
                word_length=choose_word_length()},
        "2" => println!("Modifying language"),
        "3" => println!("Goodbye!"),
        _ => println!("Invalid option"),
    }
}

fn choose_word_length() -> i32 {
    println!("Please enter the desired word length for the game (between 2 and 20):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match input.parse::<i32>() {
        Ok(word_length) => {
            if word_length < 2 || word_length > 20 {
                println!("Invalid input. Please enter a number between 2 and 20.");
                choose_word_length()
            } else {
                word_length
            }
        },
        Err(_) => {
            println!("Invalid input. Please enter a number between 2 and 20.");
            choose_word_length()
        }
    }
}





struct Words {
    size: usize,
    possiblities: Vec<Vec<i8>>,
    language: String,
    words: Vec<String>,
    remaining_words :Vec<String>
}

impl Words {
    fn new(size: usize, language: &str) -> Words {
        let words = Self::import_words(&language.to_string(), size);
        let remaining_words=words.clone();
        let possiblities = generate_possibilities(size);
        let language=language.to_string();
        Words { size , possiblities, language , words,remaining_words }
    }

    fn import_words(language: &String, size: usize) -> Vec<String> {
        let path = std::path::Path::new(FILE_PATH_EN);
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);

        reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .filter(|word| word.len() == size)
        .collect()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn words(&self) -> &Vec<String> {
        &self.words
    }

    fn count(&self) -> usize {
        self.words.len()
    }

    fn start(&mut self, first_reply: Reply){
        let first_reply_sec=self.input_sequence();

        println!("{:?}",first_reply_sec);
        let word=self.find_best(first_reply);
        print!("best word : {} with E(.)={} ",word.0,word.1);


        //self.words=self.elimine(word.chars().collect(),inpu);

    }

    fn input_sequence(&self) -> Vec<i32> {
        let mut result = vec![];
        loop {
            println!("Enter a sequence of {} numbers (only 0, 1, or 2): ", self.size);
            let mut sequence = String::new();
            io::stdin().read_line(&mut sequence).unwrap();
            let sequence = sequence.trim();
            if sequence.len() != self.size {
                println!("\x1B[31mInvalid input, the sequence must be of length {}\x1B[0m", self.size);
                continue;
            }
            let mut invalid_input = false;
            for i in sequence.chars() {
                match i {
                    '2' => {
                        result.push(2);
                    },
                    '1' => {
                        result.push(1);
                    },
                    '0' => {
                        result.push(0);
                    },
                    _ => {
                        println!("\x1B[31mInvalid input, only 0, 1, or 2 are allowed\x1B[0m");
                        result.clear();
                        invalid_input = true;
                        break;
                    },
                }
            }
            if !invalid_input {
                break;
            }
        }
        result
    }



    fn find_best(&self, reply : Reply) -> (&String,i32){
        let mut max_esperance=0;
        let mut best_word:&String=&self.words[0];
        for _word in self.words.iter() {
            let esperance=Self::compute_esperance(self,_word);
            if esperance>max_esperance{
                max_esperance=esperance;
                best_word=_word;
            }
            //print!("E({})={}\n\n",_word,esperance);
        }
        (best_word,max_esperance)
    }

    fn compute_esperance(&self, word : &String) -> i32{
        let wordChar = word.chars().collect();
        let mut esperance=0;
        let len_words=self.words.len();
        for _possiblities in self.possiblities.iter() {
            let mots_restant=self.elimine(&wordChar,_possiblities.to_vec()).len();
            esperance+= mots_restant*(len_words-mots_restant);


            //print!("    {:?},{}\n",_possiblities,esperance);

        }
        esperance=esperance / len_words;
        esperance as i32
    }

    fn elimine(&self,suggestion :&Vec<char>, reply: Vec<i8>) -> Vec<String>{
        let mut words_reply=self.words.clone();


        for(char_i,rep_i) in suggestion.iter().zip(reply.iter()){

            //println!("      item1: {} item2: {}", char_i,rep_i);
        }

        let letres_presente = suggestion.into_iter()
            .zip(reply.into_iter())
            .filter(|(_, rep_i)| *rep_i == 1 || *rep_i == 2)
            .map(|(char_i, _)| char_i)
            .collect::<Vec<_>>();


        //println!("      {:?}",letresPresente);
        words_reply=words_reply.into_iter()
            .filter(|word| {
                letresPresente.iter().all(|letter| word.contains(letter.to_string().as_str()))
            })
            .collect();

        //println!("      {:?}",words_reply);

        words_reply
    }


}

#[derive(Debug)]
enum ReplyType {
    Correct,
    WrongSpot,
    NotInTheWorld
}
struct Reply {
    suggestion: Vec<char>,
    reply : Vec<i8>
}

impl Reply {
    fn new(suggestion :String, reply : Vec<i8>) -> Reply {
        let size=reply.len();
        if suggestion.len() != size {panic!("Error wrong len")}
        Reply { suggestion: suggestion.chars().collect(), reply }
    }

}



//     Correct       -> 2
//     WrongSpot     -> 1
//     NotInTheWorld -> 0

fn generate_possibilities(n: usize) -> Vec<Vec<i8>> {
    let possibilities = vec![0, 1, 2];
    if n == 0 {
        return vec![vec![]];
    }
    let mut result = Vec::new();
    for combination in generate_possibilities(n - 1).iter() {
        for possibility in possibilities.iter() {
            let mut new_combination = combination.clone();
            new_combination.push(*possibility as i8);
            result.push(new_combination);
        }
    }
    result
}












