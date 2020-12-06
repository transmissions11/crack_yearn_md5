use md5;

fn main() {
    let goal_hash = "dbba1bfe930d953cabcc03d7b6ab05e6";

    let length = 17;

    let alphabet = "bdeilmost"
        .split("")
        .map(String::from)
        .filter(|letter| !letter.is_empty())
        .collect::<Vec<String>>();

    let mut words = alphabet.clone();

    for _ in 1..length {
        let mut temp: Vec<String> = Vec::new();

        for word in words.iter() {
            for letter in alphabet.iter() {
                let new_word = format!("{}{}", word, letter);

                println!("{}", new_word);
                temp.push(new_word);
            }
        }

        words = temp;
    }

    println!("Words len: {}", words.len());

    for word in words.iter() {
        let merged = format!(
            "{}{}",
            word, "........................................................!1"
        );

        println!("Merged {}", merged);

        let hash = format!(
            "{:x}",
            md5::compute(
                format!(
                    "{:x}",
                    md5::compute(
                        format!(
                            "{:x}",
                            md5::compute(
                                format!("{:x}", md5::compute(merged.clone().into_bytes()))
                                    .into_bytes()
                            )
                        )
                        .into_bytes(),
                    )
                )
                .into_bytes(),
            )
        );

        if hash == goal_hash {
            println!("We found it: {}", word);
            return;
        }
    }
}
