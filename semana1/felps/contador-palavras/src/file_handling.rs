use std::process::exit;

use std::env;
use std::fs::read_to_string;
use std::io::ErrorKind;

// abre arquivo, fazendo tratamento de erro
pub fn get_file_content(f_name: String) -> Option<String> {
    let content = match read_to_string(&f_name) {
        Ok(n) => Some(n),
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => println!("Arquivo não encontrado!"), // especifica se arquivo n existe
                _ => println!("Erro ao abrir o arquivo!"),
            }
            None
        }
    };
    return content;
}

pub fn get_args() -> String {
    // pega os argumentos passados
    let args: Vec<String> = env::args().collect();
    // trata
    match args.len() {
        1 => {
            println!("Insira o nome de um arquivo!");
            exit(1);
        }
        2 => args[1].clone(),
        _ => {
            println!("Insira somente um arquivo!");
            exit(1);
        }
    }
}
