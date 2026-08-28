use std::io;

pub fn ler_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

pub fn trata_input(input: String) -> Result<Vec<i32>, String> {
    let mut numbers: Vec<i32> = Vec::new();

    // checagem de se o vetor tem só numeros
    for elem in input.split_whitespace() {
        match elem.parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => return Err(String::from("insira apenas números inteiros!")),
        }
    }

    if numbers.is_empty() {
        return Err(String::from("insira no mínimo um número!"));
    }
    Ok(numbers)
}
