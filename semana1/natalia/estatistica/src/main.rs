use std::io;

fn str_to_int_vec(entrada: &str) -> Vec<i32> {
    let mut valores: Vec<i32> = Vec::new();

    for parte in entrada.trim().split_whitespace() {
        let numero = parte.parse::<i32>().unwrap();
        valores.push(numero);
    }

    valores
}

fn qtd_valores(numeros: &Vec<i32>) -> i32 {
    numeros.len() as i32
}

fn soma(numeros: &Vec<i32>) -> i32 {
    numeros.iter().sum()
}

fn menor_valor(numeros: &Vec<i32>) -> Option<i32> {
    numeros.iter().min()
}

fn maior_valor(numeros: &Vec<i32>) -> Option<i32> {
    numeros.iter().max()
}

fn media(numeros: &Vec<i32>) -> Option<f64> {
    if numero.is_empty {
        return None;
    }

    let mut soma = numeros.iter().sum()::<i32>();
    let mut qtdValores = numeros.len();

    Some(soma as f64 / qtdValores as f64)
}

fn main() {
    let mut entrada_user = String::new();

    io::stdin()
        .read_line(&mut entrada_user)
        .expect("falha ao ler a entrada");

    let valores = str_to_int_vec(&entrada_user);

    if valores.is_empty() {
        println!("entrada vazia");
        return;
    }

    println!("Quantidade: {}", qtd_valores(&valores));
    println!("Soma: {}", soma(&valores));
    println!("Media: {:.2}", media(&valores).unwrap());
    println!("Mínimo: {}", menor_valor(&valores).unwrap());
    println!("Maximo: {}", maior_valor(&valores).unwrap());
}
