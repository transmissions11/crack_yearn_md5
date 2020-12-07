use num_format::{Locale, ToFormattedString};
use std::thread;
use std::time::Duration;

fn hash_md5(input: String) -> String {
    format!("{:x}", md5::compute(input.into_bytes()))
}

fn main() {
    let goal_hash = "dbba1bfe930d953cabcc03d7b6ab05e";

    let length = 17;

    let alphabet = "bdeilmost-"
        .split("")
        .map(String::from)
        .filter(|letter| !letter.is_empty())
        .collect::<Vec<String>>();

    let mut words = alphabet.clone();

    for phase in 1..length {
        let mut temp: Vec<String> = Vec::new();

        for (index, word) in words.iter().enumerate() {
            for letter in alphabet.iter() {
                let new_word = format!("{}{}", word, letter);

                temp.push(new_word);
            }

            if index != 0 && ((index % 1000000) == 0) {
                println!(
                    "Completed Phase {}/{}'s sub-phase {}/{}",
                    phase,
                    length,
                    index.to_formatted_string(&Locale::en),
                    words.len().to_formatted_string(&Locale::en),
                );
            }
        }

        words = temp;
    }

    let length = words.len();
    println!("Length of words: {}", length);

    thread::sleep(Duration::from_secs(2));

    for (attempts, word) in words.iter().enumerate() {
        let merged = format!(
            "{}{}",
            word, "........................................................!1"
        );

        let hash = hash_md5(hash_md5(hash_md5(hash_md5(merged.clone()))));

        println!(
            "Attempts: {}/{}, Hash: {}, Text: {}",
            attempts, length, hash, merged
        );

        if hash == goal_hash {
            println!("We found it: {}", word);

            return;
        }
    }
}
