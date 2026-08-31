use crate::enums::comands::Comando;
use std::io;

pub fn ler_input() -> String {
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => (),
        Err(e) => println!("Erro ao ler o input: {}\nTente Novamente", e),
    }

    buf
}

pub fn pega_comando() -> Result<Comando, String> {
    let buf = ler_input();

    // separar em partes em um vetor
    let mut partes: Vec<String> = Vec::new();
    for elem in buf.split_whitespace() {
        partes.push(elem.to_string());
    }

    // tratamento dos comandos
    if partes.is_empty() {
        return Err(String::from("Insira no mínimo um comando!"));
    }

    match partes[0].to_uppercase().as_str() {
        "SET" => {
            if partes.len() != 3 {
                return Err(String::from("Siga o padrão \"SET <nome> <valor>\""));
            }

            match partes[2].parse() {
                Err(_) => Err(String::from("Insira um valor numérico para ser setado!")),
                Ok(valor) => Ok(Comando::Set((partes[1].clone(), valor))),
            }
        }

        "ADD" => {
            if partes.len() != 3 {
                return Err(String::from("Siga o padrão \"ADD <nome> <valor>\""));
            }

            match partes[2].parse() {
                Err(_) => Err(String::from(
                    "Insira um valor numérico para ser adicionado!",
                )),
                Ok(valor) => Ok(Comando::Add((partes[1].clone(), valor))),
            }
        }

        "SUB" => {
            if partes.len() != 3 {
                return Err(String::from("Siga o padrão \"SUB <nome> <valor>\""));
            }

            match partes[2].parse() {
                Err(_) => Err(String::from("Insira um valor numérico para ser subtraido!")),
                Ok(valor) => Ok(Comando::Sub((partes[1].clone(), valor))),
            }
        }

        "PRINT" => {
            if partes.len() == 2 {
                Ok(Comando::Print(Some(partes[1].clone())))
            } else if partes.len() == 1 {
                Ok(Comando::Print(None))
            } else {
                Err(String::from("Siga o padrão \"PRINT <nome>\" ou \"PRINT\""))
            }
        }

        "QUIT" => {
            if partes.len() != 1 {
                Err(String::from("Siga o padrão \"QUIT\""))
            } else {
                Ok(Comando::Quit)
            }
        }

        _ => Err(String::from("Insira um comando válido!")),
    }
}
