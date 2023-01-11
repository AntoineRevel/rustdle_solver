use std::fs::File;
use std::io::{BufRead, BufReader};

static FILE_PATH_EN : &str = "data/words.txt";

struct Words {
    size: usize,
    language: String,
    words: Vec<String>,

}

impl Words {
    fn new(size: usize, language: String) -> Words {
        let words = Self::import_words(&language, size);
        Words { size , language , words }
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
    fn start(&self, first_reply: Reply){
        self.find_best(first_reply);
    }

    fn find_best(&self, reply : Reply)-> String {

    }

}
enum ReplyType {
    Correct,
    WrongSpot,
    NotInTheWorld
}
struct Reply {
    suggestion: Vec<char>,
    reply : Vec<ReplyType>
}

impl Reply {
    fn new(suggestion :String, reply : Vec<ReplyType>) -> Reply {
        let size=reply.len();
        if suggestion.len() != size {panic!("Error wrong len")}
        Reply { suggestion: suggestion.chars().collect(), reply }
    }
}

pub fn start_game() {
    let words=Words::new(12, "en".to_string());
    print!("{:?}",words.count())
}

impl ReplyType{
    
}

