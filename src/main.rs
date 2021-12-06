use std::io;
use std::fs;
use std::ops::Sub;
use std::collections::HashSet;
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
    let word = words.choose(&mut thread_rng());
    word.cloned()
}

fn get_sprites() -> [String; 7] {
    [
        "_____\n|   |\n|   O\n|  /|\\\n|  / \\\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /|\\\n|  /\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /|\\\n|\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|  /|\n|\n|\n|".to_string(),
        "_____\n|   |\n|   O\n|   |\n|\n|\n|".to_string(),
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
    loop {
        println!("enter a guess: ");
        match stdin.read_line(&mut buffer) {
            Ok(_i) => {
                let mut input = buffer.remove(0);
                if input.is_ascii_alphabetic() {
                    input.make_ascii_lowercase();
                    guesses.push(input.clone());
                    return input;
                } else {
                    println!("input is invalid");
                    continue;
                };
            },
            Err(_) => {
                println!("input is invalid");
                continue;
            },
        };
    };
}

fn process_guessing(guess :&char, tries :usize, secret_letters :&mut HashSet<char>) -> usize {
    match secret_letters.remove(guess) {
        true => {
            println!("{} is a secret letter", guess);
            tries
        },
        false => {
            println!("{} is not a secret letter", guess);
            tries.sub(1)
        },
    }
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
        tries = process_guessing(&guess, tries, &mut secret_letters);
    };
    println!("Guesses: {:?}", guesses);
    println!("{}", sprites[tries]);
    println!("    {}", secret_word(&word, &secret_letters));

    if tries == 0 && !secret_letters.is_empty() {
        println!("You lost, the word was {}", word);
    } else {
        println!("You Won, the word was {}", word);
    };
}

fn main() {
    match get_word("src/words.txt") {
        Some(word) => game(word),
        None => println!("file couldn't be found."),
    };
}
