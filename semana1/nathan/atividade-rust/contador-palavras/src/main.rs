use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Error};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut map_words: HashMap<String, i32> = HashMap::new();

    if args.len() < 2 {
        return Err(Error::other("Nenhum arquivo foi informado"));
    }

    let path = &args[1];
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total_lines = 0;

    for line in reader.lines() {
        total_lines += 1;

        let line_content = line?;
        for word in line_content.split_whitespace() {
            *map_words.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    
    let mut min: i32 = i32::MIN;
    let mut more_frequent_word: String = String::from("");
    let mut total_words: i32 = 0;
    for (word, count) in &map_words {
        if *count > min {
            more_frequent_word = word.to_string();
            min = *count;
        }
        total_words += *count;
    }

    println!("O arquivo tem {} linhas.", total_lines);
    println!("Total de palavras: {}", total_words);
    println!("Palavra mais frequente: {} ({})", more_frequent_word, min);
    Ok(())
}