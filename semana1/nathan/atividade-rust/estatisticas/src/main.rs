use std::io;

fn quantidade(numeros: &Vec<i32>) -> usize {
    numeros.len()
}

fn soma(numeros: &Vec<i32>) -> i32 {
    numeros.iter().sum()
}

fn maior(numeros: &Vec<i32>) -> i32 {
    *numeros.iter().max().unwrap()
}

fn menor(numeros: &Vec<i32>) -> i32 {
    *numeros.iter().min().unwrap()
}

fn media(numeros: &Vec<i32>) -> f32 {
    let total = soma(numeros);
    let count = quantidade(numeros);
    total as f32 / count as f32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numeros: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if numeros.is_empty() {
        println!("Nenhum número válido foi fornecido.");
        return;
    }
    
    let contador = quantidade(&numeros);
    let total = soma(&numeros);
    let menor = menor(&numeros);
    let maior = maior(&numeros);
    let media = media(&numeros);

    println!("Saida: {}", input);
    println!("Quantidade: {}", contador);
    println!("Soma: {}", total);
    println!("Media: {}", media);
    println!("Menor: {}", menor);
    println!("Maior: {}", maior);
}
