use std::io;
use std::collections::HashMap;

enum Command {
    SET { nome: String, valor: i32 },
    ADD { nome: String, valor: i32 },
    SUB { nome: String, valor: i32 },
    PRINTVAR(String),
    PRINTALL,
    QUIT,
}

fn interpretator(args: Vec<String>) -> Option<Command> {
    if args.is_empty() {
        return None;
    }

    match args[0].to_uppercase().as_str() {
        "SET" | "ADD" | "SUB" => {
            if args.len() < 3 {
                eprintln!("Erro: O comando '{}' precisa de nome e valor.", args[0]);
                return None;
            }

            let valor = match args[2].parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Erro: '{}' não eh um inteiro valido.", args[2]);
                    return None;
                }
            };

            let nome = args[1].clone();

            match args[0].to_uppercase().as_str() {
                "SET" => Some(Command::SET { nome, valor }),
                "ADD" => Some(Command::ADD { nome, valor }),
                "SUB" => Some(Command::SUB { nome, valor }),
                _ => None,
            }
        }
        "PRINT" => {
            if args.len() >= 2 {
                Some(Command::PRINTVAR(args[1].clone()))
            } else {
                Some(Command::PRINTALL)
            }
        }
        "QUIT" => Some(Command::QUIT),
        _ => {
            eprintln!("Erro: Comando não reconhecido.");
            None
        }
    }
}

fn exec_commands(cmd: Command, memory: &mut HashMap<String, i32>) -> bool{
    match cmd {
        Command::SET { nome, valor } => { memory.insert(nome, valor); },
        Command::ADD { nome, valor } => { *memory.entry(nome).or_insert(0) += valor;},
        Command::SUB { nome, valor } => { *memory.entry(nome).or_insert(0) -= valor;},
        Command::PRINTVAR(nome) => if let Some(v) = memory.get(&nome) { println!("{} = {}", nome, v);},
        Command::PRINTALL => for (k, v) in memory { println!("{} = {}", k, v);},
        Command::QUIT => return false,
    }
    
    true
}

fn main() {
    let mut memory: HashMap<String, i32> = HashMap::new();
    loop {
        let mut input: String = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Erro ao ler a entrada do terminal.");
            continue;
        }

        let arguments: Vec<String> = input.split_whitespace().map(String::from).collect();

        if let Some(cmd) = interpretator(arguments) {
            if !exec_commands(cmd, &mut memory) { break; }
        }
    }
}
