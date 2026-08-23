mod input;
mod calculo;

use input::*;
use calculo::*;



fn main() {
    println!("Escreva uma lista de números inteiros!");
    let input = ler_input();

    let numeros = match trata_input(input){
        Ok(numbers) =>  numbers,
        Err(e) => {
            println!("Erro: {}", e);
            return;
        }
    };

    println!("{:?}", numeros);
    
    let quant = calcula_quantidade(&numeros);
    let soma = calcula_soma(&numeros);
    let media: f32 = soma as f32/quant as f32;
    let max_min: (i32, i32) = calcula_max_min(&numeros);

    println!("Quantidade: {}
Soma: {}
Media: {}
Minimo: {}
Maximo: {}", quant, soma, media, max_min.1, max_min.0);
}
