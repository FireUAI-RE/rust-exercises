use std::env;
use std::collections::HashMap;
use std::io::{Result, Error};


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut map_words: HashMap<String, i32> = HashMap::new();

    if args.len() < 3 {
        return Err(Error::other("Uso: cargo run -- <texto> <palavra>"));
    }

    let text = &args[1];
    let word = &args[2];

    count_words(text, &mut map_words);

    println!("Caracteres: {}", total_chars(text));
    println!("Palavras: {}", total_words(&map_words));
    println!("Maior palavra: {}", longest_word(text));
    println!("Ocorrências de {}: {}", word, n_times(&map_words, word));

    Ok(())
}

fn count_words(text: &str, words: &mut HashMap<String, i32>) {
    for word in text.split_whitespace() {
        let w = word;
        if w.is_empty() {
            continue;
        }
        *words.entry(w.to_lowercase()).or_insert(0) += 1;
    }
}

fn total_chars(text: &str) -> usize {
    text.chars().count()
}

fn total_words(words: &HashMap<String, i32>) -> i32 {
    let mut total_words: i32 = 0;
    for (_word, count) in words {
        total_words += *count;
    }
    total_words
}

fn longest_word(text: &str) -> &str {
    let mut longest = "";
    for word in text.split_whitespace() {
        let w = word;
        if w.chars().count() >= longest.chars().count() {
            longest = w;
        }
    }
    longest
}

fn n_times(words: &HashMap<String, i32>, target: &str) -> i32 {
    words.get(&target.to_lowercase()).copied().unwrap_or(0)
}

