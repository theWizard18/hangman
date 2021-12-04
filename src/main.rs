use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn get_secret_word(file_path: &str) -> Option<String> {
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

fn main() {
    match get_secret_word("src/words.txt") {
        Some(secret_word) => println!("{}", secret_word),
        None => println!("file couldn't be found."),
    };
}
