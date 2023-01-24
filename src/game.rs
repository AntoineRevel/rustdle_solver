use std::fs::File;
use std::io::{BufRead, BufReader};

static FILE_PATH_EN : &str = "data/words.txt";

pub fn start_game() {
    let mut words =Words::new(4, "en");
    let reply = Reply::new("adds",vec![0, 1, 2,0]);

    Words::start(&mut words, reply);

    //print!("{:?}\n",words.words);
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
        let word=self.find_best(first_reply);
        print!("best word : {}",word.0);
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
            let mots_restant=self.elimine(&wordChar,_possiblities.to_vec());
            esperance+= mots_restant*(len_words-mots_restant);


            //print!("    {:?},{}\n",_possiblities,esperance);

        }
        esperance=esperance / len_words;
        esperance as i32
    }

    fn elimine(&self,suggestion :&Vec<char>, reply: Vec<i8>) -> usize{
        let mut words_reply=self.words.clone();


        for(char_i,rep_i) in suggestion.iter().zip(reply.iter()){

            //println!("      item1: {} item2: {}", char_i,rep_i);
        }

        let letresPresente = suggestion.into_iter()
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












