mod counting_operations;
mod file_handling;

use counting_operations::*;
use file_handling::*;

use std::process::exit;

fn main() {
    let file_name = get_args();
    let content = match get_file_content(file_name) {
        Some(n) => n,
        None => exit(1),
    };

    let word_list = create_word_list(&content);
    let line_count = count_lines(&content);
    let word_count = count_words(&word_list);
    let most_freq = get_most_frequent(&word_list);

    println!(
        "Linhas: {}
Palavras: {}
Palavra mais frequente: {} ({})",
        line_count, word_count, most_freq.0, most_freq.1
    );
}
