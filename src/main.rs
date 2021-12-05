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

fn get_sprites() -> [String; 7] {
    [
        "_____\n|   |\n|   O\n|  /|\\\n|  / \\\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /|\\\n|  /\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /|\\\n|\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /|\n|\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /\n|\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|\n|\n|\n|".to_string(),
        "_____\n|   |\n|\n|\n|\n|\n|".to_string()
    ]
}

fn get_secret_letters(word :&String) -> HashSet<char> {
    let mut letters = HashSet::new();
    for l in word.chars() {
        letters.insert(l.clone());
    };
    letters
}

fn secret_word(word :&String, secret_letters :&HashSet<char>) -> String {
    let mut secret_word = String::new();
    for l in word.chars() {
        if secret_letters.contains(&l) {
            secret_word.push('_');
        } else {
            secret_word.push(l.clone());
        };
    };
    secret_word
}

fn prompt(guesses: &mut Vec<char>) -> char {
    let mut buffer = String::new();
    let stdin = io::stdin();
    print!("enter a guess: ");
    match stdin.read_line(&mut buffer) {
        Ok(_i) => {
            let input = buffer.remove(0);
            guesses.push(input.clone());
            input
        },
        Err(_) => ' ',
    }
}

fn process_guessing(guess :&char, tries :&mut usize, secret_letters :&mut HashSet<char>) {
    match secret_letters.remove(guess) {
        true => println!("{} is a secret word", guess),
        false => {
            println!("{} is not a secret word", guess);
            tries -= 1;
        }
    };
}

fn game(word: String) {
    let mut tries = 6;
    let sprites = get_sprites();
    let mut guess: char;
    let mut guesses :Vec<char> = Vec::new();
    let mut secret_letters = get_secret_letters(&word);
    while tries != 0 && !secret_letters.is_empty() {
        println!("Guesses: {:?}", guesses);
        println!("{}", sprites[tries]);
        println!("    {}", secret_word(&word, &secret_letters));
        guess = prompt(&mut guesses);
        process_guessing(&guess, &mut tries, &mut secret_letters);
    };
    if tries == 0 || secret_letters.is_empty() {
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
