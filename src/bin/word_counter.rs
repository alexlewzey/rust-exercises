use std::collections::HashMap;

fn count_words(sentence: &str) -> Vec<(String, u32)> {
    let words = sentence.split_ascii_whitespace();
    let mut counts: HashMap<String, u32> = HashMap::new();
    for word in words {
        let word_norm: String = word
            .to_lowercase()
            .chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect();
        *counts.entry(word_norm).or_insert(0) += 1
    }
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by_key(|(k, v)| (std::cmp::Reverse(*v), k.clone()));
    counts
}

fn main() {
    let sentence = "Hello Mole! You are a Mole! Hello Ted!";
    let counts = count_words(sentence);
    let max_len = counts.iter().map(|(k, _)| k.chars().count()).max().unwrap();
    println!("{}", "-".repeat(max_len + 6));
    println!("{:<max_len$} {}", "word", "count");
    println!("{}", "-".repeat(max_len + 6));
    for (k, v) in counts {
        println!("{k:<max_len$} {v}")
    }
}
