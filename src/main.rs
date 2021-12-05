use std::collections::HashSet;
use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn get_word(file_path: &str) -> Option<String> {
    let content = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(_) => String::new(),
    };
    let mut words = Vec::new();
    for v in content.lines() {
        words.push(String::from(v));
    };
    words.shuffle(&mut thread_rng());
    let word = words.pop();
    word
}

fn get_secret_letters(word :&String) -> HashSet<String> {
    let mut letters = HashSet::new();
    for l in word {
        letters.insert(l.to_string());
    };
    letters
}

fn get_sprites() -> (String, String, String, String, String, String, String) {
    (
        "_____\n|   |\n|   O\n|  /|\\\n|  / \\\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /|\\\n|  /\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /|\\\n|\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /|\n|\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /\n|\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|\n|\n|\n|".to_string(),
        "_____\n|   |\n|\n|\n|\n|\n|".to_string()
    )
}
        
fn game(word: mut String) {
    let mut tries = 6;
    let sprites = get_sprites();
    let mut guesses = Vec::new()
    let mut secret_letters = get_secret_letters(&word);
    while assert_ne!(tries, 0) && assert!(!secret_letters.is_empty()) {
        println!("Guesses: {:?}", guesses);
        println!("{}", sprites.tries);
    }
}

fn main() {
    match get_word("src/words.txt") {
        Some(word) => println!("{}", word),
        None => println!("file couldn't be found."),
    };
}
