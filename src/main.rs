use itertools::Itertools;
use std::thread;
use std::time::Duration;

fn hash_md5(input: String) -> String {
    format!("{:x}", md5::compute(input.into_bytes()))
}

fn main() {
    let goal_hash = "dbba1bfe930d953cabcc03d7b6ab05e6";

    let alphabet = "bdeilmostu"
        .split("")
        .map(String::from)
        .filter(|letter| !letter.is_empty());

    let words = alphabet.combinations_with_replacement(17).collect_vec();

    println!("Length of words: {}", words.len());

    thread::sleep(Duration::from_secs(2));

    for word in words.iter() {
        let joined: String = word.join("");
        let merged = format!(
            "{}{}",
            joined, "........................................................!1"
        );

        println!("{}", merged);

        let hash = hash_md5(hash_md5(hash_md5(hash_md5(merged))));

        if hash == goal_hash {
            println!("We found it: {}", joined);
            thread::sleep(Duration::from_secs(100));
            return;
        }
    }
}
