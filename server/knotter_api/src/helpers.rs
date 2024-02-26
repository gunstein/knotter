use crate::domain::errors::my_error::MyError;
use regex::Regex;
use chrono::{self, Utc};
use rand::Rng;
use rand::seq::SliceRandom;

pub fn get_after_dashdash(s: &str) -> Option<&str> {
    let mut parts = s.split("--");
    parts.next()?;  // consume everything before "--"
    parts.next()    // get everything after "--"
}

pub fn process_globe_id(globe_id: &str) -> Result<String, MyError> {
    let globe_id = globe_id.to_lowercase();

    if globe_id.len() > 12 {
        return Err(MyError::ValidationError("globe_id should not be longer than 12 characters".to_string()));
    }

    let pattern = r"^[bcdfghjklmnpqrstvwxyz][aeiou][bcdfghjklmnpqrstvwxyz][aeiou]\d{2}[bcdfghjklmnpqrstvwxyz][aeiou][bcdfghjklmnpqrstvwxyz][aeiou]$";
    let re = Regex::new(pattern).unwrap();
    if !re.is_match(&globe_id) {
        return Err(MyError::ValidationError("globe_id is not valid.".to_string()));
    }

    Ok(globe_id)
}

pub fn generate_timestamp() -> String {
    let now = Utc::now();
    (now.timestamp_subsec_nanos() as i64 + now.timestamp() * 1_000_000_000).to_string()
}

pub fn generate_globe_id() -> String {
    // Define vowels and consonants
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 
        'h', 'j', 'k', 'l', 'm', 
        'n', 'p', 'q', 'r', 's', 
        't', 'v', 'w', 'x', 'y', 'z'
    ];

    // Generate two words
    let word1 = generate_word(&vowels, &consonants);
    let word2 = generate_word(&vowels, &consonants);

    // Generate a random digit
    let digit1: u8 = rand::thread_rng().gen_range(0..10);
    let digit2: u8 = rand::thread_rng().gen_range(0..10);

    // Combine the words with the digit
    format!("{}{}{}{}", word1, digit1, digit2, word2)
}

pub fn generate_word(vowels: &[char], consonants: &[char]) -> String {
    let mut rng = rand::thread_rng();
    let mut word = String::new();

    for i in 0..4 {
        let letters = if i % 2 == 0 { consonants } else { vowels };
        let &letter = letters.choose(&mut rng).unwrap();
        word.push(letter);
    }

    word
}
