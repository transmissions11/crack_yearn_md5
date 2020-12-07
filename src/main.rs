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

        for word in words.iter() {
            for letter in alphabet.iter() {
                let new_word = format!("{}{}", word, letter);

                temp.push(new_word);
            }
        }

        println!("Phase {}/{}", phase, length);

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
