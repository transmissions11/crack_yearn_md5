use std::fs;

fn hash_md5(input: String) -> String {
    format!("{:x}", md5::compute(input.into_bytes()))
}

fn get_dictionary_words() -> Vec<String> {
    let contents =
        fs::read_to_string("./realwords.txt").expect("Something went wrong reading the file");

    return contents
        .lines()
        .map(|s| String::from(s.trim()))
        .collect::<Vec<String>>();
}

fn filter_by_word_length_and_charset(
    words: &[String],
    word_len: usize,
    char_set: &[String],
) -> Vec<String> {
    words
        .iter()
        .filter_map(|word| {
            let no_apostrophe = word.replace("'", "");

            let mut contains_only_char_set = true;

            for c in no_apostrophe.chars() {
                if !(char_set.contains(&c.to_string())) {
                    contains_only_char_set = false;
                    break;
                }
            }

            let is_right_len = no_apostrophe.len() == word_len;

            if is_right_len && contains_only_char_set {
                Some(no_apostrophe)
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

fn main() {
    let goal_hash = "86566bae1bb08cfe94a58cd86f391b10";

    let char_set = "bdeilmostu"
        .split("")
        .map(String::from)
        .filter(|letter| !letter.is_empty())
        .collect::<Vec<String>>();

    let words = get_dictionary_words();

    let second_words = filter_by_word_length_and_charset(&words, 4, &char_set);
    let third_words = filter_by_word_length_and_charset(&words, 2, &char_set);
    let fourth_words = filter_by_word_length_and_charset(&words, 5, &char_set);

    for second_word in second_words {
        for third_word in &third_words {
            for fourth_word in &fourth_words {
                let word_sequence =
                    format!("{}-{}-{}-{}", "its", second_word, third_word, fourth_word);

                let merged = format!(
                    "{}{}",
                    word_sequence, "........................................................!1"
                );

                #[cfg(debug_assertions)]
                println!("{}", merged);

                let hash = hash_md5(hash_md5(merged.clone()));

                if hash == goal_hash {
                    println!("We found the password: {}", merged);

                    return;
                }
            }
        }
    }
}
