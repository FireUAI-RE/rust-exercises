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
    let mut qtd = 0;
    // _ faz o comp ignorar. se coloca uma variável e nao usa, dá erro
    for _ in numeros {
        qtd += 1;
    }
    qtd
}

fn soma(numeros: &Vec<i32>) -> i32 {
    let mut total = 0;
    for &valor in numeros {
        total += valor;
    }
    total
}

fn menor_valor(numeros: &Vec<i32>) -> Option<i32> {
    if numeros.is_empty() {
        return None;
    }

    let mut menor = numeros[0];
    for &valor in numeros {
        if valor < menor {
            menor = valor;
        }
    }
    Some(menor)
}

fn maior_valor(numeros: &Vec<i32>) -> Option<i32> {
    if numeros.is_empty() {
        return None;
    }

    let mut maior = numeros[0];
    for &valor in numeros {
        if valor > maior {
            maior = valor;
        }
    }
    Some(maior)
}

fn media(numeros: &Vec<i32>) -> Option<f64> {
    let qtd = qtd_valores(numeros);

    if qtd == 0 {
        return None;
    }

    let total = soma(numeros);

    Some(total as f64 / qtd as f64)
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
