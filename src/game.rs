use std::fs::File;
use std::io::{BufRead, BufReader};

static FILE_PATH_EN : &str = "data/words.txt";

pub fn start_game() {
    let mut words =Words::new(4, "en");
    let reply = Reply::new("adds",vec![0, 1, 2,0]);

    Words::start(&mut words, reply);
    //print!("{:?}\n",pos);
    //print!("{}",pos.len());

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
        self.find_best(first_reply);
    }

    fn find_best(&self, reply : Reply) {
        let mut max_esperance=0;
        let mut best_word:&String=&self.words[0];
        for _word in self.words.iter() {
            let esperance=Self::compute_esperance(self,_word);
            if esperance>max_esperance{
                max_esperance=esperance;
                best_word=_word;
            }
            print!("E({})={}\n",_word,esperance);
        }
    }

    fn compute_esperance(&self, word : &String) -> i32{
        let wordChar = word.chars().collect();
        let mut esperance=0;
        for _possiblities in self.possiblities.iter() {
            esperance+=1;
            self.elimine(&wordChar,_possiblities.to_vec());

            //print!("{:?},{}\n",_possiblities,esperance);

        }
        esperance
    }

    fn elimine(&self,suggestion :&Vec<char>, reply: Vec<i8>) -> usize{
        let mut words_reply=self.words.clone();




        words_reply.len()
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
    fn new(suggestion :&str, reply : Vec<i8>) -> Reply {
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












