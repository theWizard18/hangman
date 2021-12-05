use std::collections::HashSet;
use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

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

fn get_secret_letters(word :&String) -> HashSet<char> {
    let mut letters = HashSet::new();
    for l in word {
        letters.insert(l as char);
    };
    letters
}

fn secret_word(word :&String, secret_letters :&HashSet<char>) -> String {
    let mut secret_word = String::new();
    for l in word {
        if secret_letters.contains(l) {
            secret_word.push('_');
        } else {
            secret_word.push(l);
        };
    };
    secret_word
}

fn prompt(guesses: &mut Vec<String>) -> String {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    print!("enter a guess: ");
    match stdin.read_line(&mut buffer) {
        Ok(i) => {
            guesses.push(i);
            i
        },
        Err(_) => String::new(),
    }
}

fn process_guessing(guess :&String, secret_letters :&mut HashSet<char>) {
    match secret_letters.remove(&guess) {
        true => println!("{} is a secret word", guess);
        false => println!("{} is not a secret word", guess);
    };
}

fn game(word: mut String) {
    let mut tries = 6;
    let sprites = get_sprites();
    let mut guess = String::new();
    let mut guesses = Vec::new();
    let mut secret_letters = get_secret_letters(&word);
    while assert_ne!(tries, 0) && assert!(!secret_letters.is_empty()) {
        println!("Guesses: {:?}", guesses);
        println!("{}", sprites.tries);
        println!("    {}", secret_word(&word, &secret_letters));
        guess = prompt(&guesses);
        process_guessing(&guess, &mut secret_letters);
    };
    if assert_eq!(tries, 0) || assert!(secret_letters.is_empty()) {
        println!("You lost");
    } else {
        println!("You Won");
    };
}

fn main() {
    match get_word("src/words.txt") {
        Some(word) => game(word),
        None => println!("file couldn't be found."),
    };
}
