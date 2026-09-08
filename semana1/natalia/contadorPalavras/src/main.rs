use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn main() {
    let argumentos: Vec<String> = env::args().collect();
    let caminho_arquivo = match argumentos.get(1) {
        Some(arg) => arg,
        None => {
            eprintln!("Uso: {} <arquivo>", argumentos[0]);
            process::exit(1);
        }
    };
    let conteudo = fs::read_to_string(caminho_arquivo).unwrap_or_else(|err| {
        eprintln!("erro ao ler '{}': {}", caminho_arquivo, err);
        process::exit(1);
    });
    let total_linhas = conteudo.lines().count();
    let mut total_palavras = 0;
    let mut qtd_palavras: HashMap<String, usize> = HashMap::new();
    for palavra in conteudo.split_whitespace() {
        total_palavras += 1;
        let minuscula = palavra.to_lowercase();
        *qtd_palavras.entry(minuscula).or_insert(0) += 1;
    }
    let palavra_mais_frequente = qtd_palavras.iter().max_by_key(|&(_, count)| count);
    let resultado_mais_frequente = match palavra_mais_frequente {
        Some((palavra, qtd)) => format!("{} ({})", palavra, qtd),
        None => "nenhuma palavra encontrada".to_string(),
    };
    println!("Linhas: {}", total_linhas);
    println!("Palavras: {}", total_palavras);
    println!("Palavra mais frequente: {}", resultado_mais_frequente);
}
