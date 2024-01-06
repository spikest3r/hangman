use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use rand::Rng;
use std::{io, usize};

fn read_json_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let json_value: Value = serde_json::from_reader(reader)?;
    let strings: Vec<String> = json_value
        .as_array()
        .ok_or("Invalid JSON structure")?
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    Ok(strings)
}

fn generate_blank(word: String) -> Vec<char> {
    let mut i: i32 = 0;
    let mut result: Vec<char> = Vec::new();
    while i < word.len().try_into().unwrap() {
        result.push('_');
        i+=1;
    }
    return result
}

fn to_string(vec: Vec<char>) -> String {
    let mut result: String = String::new();
    for x in vec {
        result.push(x);
        result.push(' ');
    }
    return result
}

fn read_line() -> Result<String,&'static str> {
    let mut buffer: String = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            let buffer = buffer.trim().to_string();
            Ok(buffer)
        }
        Err(_) => {
            Err("Failed to read line.")
        }
    }
}

fn is_char_in_word(word: String, char: char, status: Vec<char>) -> (bool,Vec<i32>) {
    let arr = word.chars().into_iter();
    let mut i: i32 = 0;
    let mut result: bool = false;
    let mut index_vector: Vec<i32> = Vec::new();
    for c in arr {
        if c == char && status[i as usize]== '_' {
            result = true;
            index_vector.push(i);
        }
        i+=1;
    }
    return (result,index_vector);
}

fn main() {
    let mut words: Vec<String> = Vec::new();
    match read_json_file("assets/nouns.json") {
        Ok(strings) => {
            words = strings;
            println!("Loaded!\n\n");
        }
        Err(e) => println!("Error! {e}")
    };

    let word: &String = &words[rand::thread_rng().gen_range(1..=words.len())];
    let mut user_word: Vec<char> = generate_blank(word.clone());
    let mut guessed_chars: i32 = 0;
    let mut guesses_left: i32 = word.len() as i32 * 3;
    let mut win: bool = false;

    loop {
        let user_word_string = to_string(user_word.clone());
        println!("{user_word_string}\nNumber of guesses left: {guesses_left}");
        let mut user_input: String = String::new();
        match read_line() {
            Ok(result) => {
                user_input = result;
            }
            Err(err) => {
                println!("Error reading String! {err}");
                continue;
            }
        }
        if &user_input.len().try_into().unwrap() != 1 {
            println!("You must enter one character. (len == 1)");
            continue;
        }
        let user_input = user_input.chars().next().expect("Error!");
        guesses_left -= 1;
        let (result,index) = is_char_in_word(word.to_string(), user_input,user_word.clone());
        if result {
            for i in index {
                user_word[i as usize] = user_input;
                guessed_chars += 1;
            }
        }
        println!(); // Blank line
        if guessed_chars == word.len() as i32 {
            win = true;
            break;
        }
        if guesses_left == 0 {
            break;
        }
    }
    let text = if win {"You win!"} else {"Game over!"};
    println!("{}",text);
    println!("The word was \"{word}\"!");
}
