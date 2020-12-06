use itertools::Itertools;
use std::thread;
use std::time::Duration;

fn hash_md5(input: String) -> String {
    format!("{:x}", md5::compute(input.into_bytes()))
}

fn main() {
    let goal_hash = "dbba1bfe930d953cabcc03d7b6ab05e";

    let chars = "bdeilmostu-";

    let repeated_chars = chars.repeat(17);

    let alphabet = repeated_chars
        .split("")
        .map(String::from)
        .filter(|letter| !letter.is_empty());

    let words = alphabet.combinations(17).collect_vec();

    let length = words.len();
    println!("Length of words: {}", length);

    thread::sleep(Duration::from_secs(2));

    for (attempts, word) in words.iter().enumerate() {
        let joined: String = word.join("");

        let merged = format!(
            "{}{}",
            joined, "........................................................!1"
        );

        let hash = hash_md5(hash_md5(hash_md5(hash_md5(merged.clone()))));

        println!(
            "Attempts: {}/{}, Hash: {}, Text: {}",
            attempts, length, hash, merged
        );

        if hash == goal_hash {
            println!("We found it: {}", joined);

            return;
        }
    }
}
