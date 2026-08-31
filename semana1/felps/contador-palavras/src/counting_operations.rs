use std::collections::HashMap;

pub fn create_word_list(content: &str) -> HashMap<String, i32> {
    let mut word_list: HashMap<String, i32> = HashMap::new();
    for line in content.lines() {
        for word in line.split_whitespace() {
            let word = word.to_lowercase();
            match word_list.get(&word) {
                // palavra não estava presente
                // add ela
                None => word_list.insert(word, 1),

                // palavra já está presente
                // aumenta contador
                Some(x) => word_list.insert(word, x + 1),
            };
        }
    }
    word_list
}

pub fn count_lines(content: &str) -> usize {
    content.lines().count()
}

pub fn get_most_frequent(word_list: &HashMap<String, i32>) -> (String, i32) {
    let mut most_frequent = (String::new(), 0);

    for (word, amount) in word_list {
        if *amount > most_frequent.1 {
            most_frequent = (word.clone(), *amount);
        }
    }

    most_frequent
}

pub fn count_words(word_list: &HashMap<String, i32>) -> i32 {
    let mut word_count = 0;
    for amount in word_list.values() {
        word_count += amount;
    }
    word_count
}
