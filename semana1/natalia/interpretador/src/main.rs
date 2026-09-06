use std::collections::HashMap;
use std::io::{self, Write};

enum Instrucao {
    Definir(String, i32),
    Somar(String, i32),
    Subtrair(String, i32),
    Mostrar(String),
    MostrarTudo,
    Encerrar,
}

fn le_string(texto: &str) -> Result<Instrucao, String> {
    let palavras: Vec<&str> = texto.trim().split_whitespace().collect();

    if palavras.is_empty() {
        return Err("comando vazio".to_string());
    }

    match palavras[0].to_uppercase().as_str() {
        "SET" => {
            if palavras.len() != 3 {
                return Err("erro. usar SET <nome> <valor>".to_string());
            }
            let valor_num = palavras[2]
                .parse::<i32>()
                .map_err(|_| format!("erro. '{}' não é inteiro.", palavras[2]))?;
            Ok(Instrucao::Definir(palavras[1].to_string(), valor_num))
        }
        "ADD" => {
            if palavras.len() != 3 {
                return Err("erro. usar ADD <nome> <valor>".to_string());
            }

            let valor_num = palavras[2]
                .parse::<i32>()
                .map_err(|_| format!("erro. '{}' não é um inteiro.", palavras[2]))?;
            Ok(Instrucao::Somar(palavras[1].to_string(), valor_num))
        }
        "SUB" => {
            if palavras.len() != 3 {
                return Err("erro. usar SUB <nome> <valor>".to_string());
            }
            let valor_num = palavras[2]
                .parse::<i32>()
                .map_err(|_| format!("erro. '{}' não é inteiro.", palavras[2]))?;
            Ok(Instrucao::Subtrair(palavras[1].to_string(), valor_num))
        }
        "PRINT" => {
            if palavras.len() == 1 {
                Ok(Instrucao::MostrarTudo)
            } else if palavras.len() == 2 {
                Ok(Instrucao::Mostrar(palavras[1].to_string()))
            } else {
                Err("erro. usar PRINT ou PRINT <nome>".to_string())
            }
        }
        "QUIT" => {
            if palavras.len() != 1 {
                return Err("erro. o comando QUIT não recebe argumentos.".to_string());
            }
            Ok(Instrucao::Encerrar)
        }
        _ => Err(format!("erro. comando '{}' não reconhecido.", palavras[0])),
    }
}

fn exec(acao: Instrucao, memoria: &mut HashMap<String, i32>) -> bool {
    match acao {
        Instrucao::Definir(nome_var, valor) => {
            memoria.insert(nome_var, valor);
        }
        Instrucao::Somar(nome_var, valor) => {
            let valor_atual = memoria.entry(nome_var).or_insert(0);
            *valor_atual += valor;
        }
        Instrucao::Subtrair(nome_var, valor) => {
            let valor_atual = memoria.entry(nome_var).or_insert(0);
            *valor_atual -= valor;
        }
        Instrucao::Mostrar(nome_var) => {
            if let Some(valor) = memoria.get(&nome_var) {
                println!("{} = {}", nome_var, valor);
            } else {
                println!("erro. variável '{}' não encontrada.", nome_var);
            }
        }
        Instrucao::MostrarTudo => {
            if memoria.is_empty() {
                println!("(nenhuma variável definida)");
            } else {
                for (nome_var, valor) in memoria.iter() {
                    println!("{} = {}", nome_var, valor);
                }
            }
        }
        Instrucao::Encerrar => {
            return false;
        }
    }
    true
}

fn main() {
    let mut memoria_do_programa: HashMap<String, i32> = HashMap::new();
    let mut user_entrada = String::new();

    println!("digite o comando: (QUIT para sair).");

    loop {
        print!("> ");
        if let Err(erro_io) = io::stdout().flush() {
            eprintln!("erro de I/O: {}", erro_io);
            break;
        }

        user_entrada.clear();
        match io::stdin().read_line(&mut user_entrada) {
            Ok(0) => break,
            Ok(_) => match le_string(&user_entrada) {
                Ok(acao) => {
                    if !exec(acao, &mut memoria_do_programa) {
                        break;
                    }
                }
                Err(erro_msg) => {
                    println!("{}", erro_msg);
                }
            },
            Err(erro) => {
                eprintln!("erro {}", erro);
            }
        }
    }
}
