use std::io;
use std::collections::HashMap;

struct Student {
    name: String,
    grades: Vec<f64>,
}

enum Command {
    ADD(String),
    GRADE { nome: String, valor: f64 },
    SHOW(String),
    AVERAGE(String),
    LIST,
    QUIT,
}

fn main() {
    let mut memory: HashMap<String, Student> = HashMap::new();
    loop {
        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => {
                eprintln!("Erro ao ler a entrada do terminal.");
                continue;
            }
        }

        let arguments: Vec<String> = input.split_whitespace().map(String::from).collect();

        if let Some(cmd) = interpretator(arguments) {
            if !exec_commands(cmd, &mut memory) { break; }
        }
    }
}

fn interpretator(args: Vec<String>) -> Option<Command> {
    if args.is_empty() {
        return None;
    }

    match args[0].to_uppercase().as_str() {
        "ADD" | "SHOW" | "AVERAGE" => {
            if args.len() < 2 {
                eprintln!("Erro: O comando '{}' precisa de nome.", args[0]);
                return None;
            }

            let nome = args[1].clone();

            match args[0].to_uppercase().as_str() {
                "ADD" => Some(Command::ADD (nome)),
                "SHOW" => Some(Command::SHOW (nome)),
                "AVERAGE" => Some(Command::AVERAGE (nome)),
                _ => None,
            }
        }
        "GRADE" => {
            if args.len() < 3 {
                eprintln!("Erro: O comando '{}' precisa de nome e valor.", args[0]);
                return None;
            }

            let valor = match args[2].parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Erro: '{}' não eh um inteiro valido.", args[2]);
                    return None;
                }
            };

            let nome = args[1].clone();

            match args[0].to_uppercase().as_str() {
                "GRADE" => Some(Command::GRADE { nome, valor }),
                _ => None,
            }
        }
        "LIST" => Some(Command::LIST),
        "QUIT" => Some(Command::QUIT),
        _ => {
            eprintln!("Erro: Comando não reconhecido.");
            None
        }
    }
}

fn exec_commands(cmd: Command, memory: &mut HashMap<String, Student>) -> bool {
    match cmd {
        Command::ADD (nome) => {
            if memory.contains_key(&nome) {
                eprintln!("Erro: estudante '{}' já existe.", nome);
            } else {
                memory.insert(nome.clone(), Student { name: nome, grades: Vec::new() });
            }
        }
        Command::GRADE { nome, valor } => {
            if !(0.0..=10.0).contains(&valor) {
                eprintln!("Erro: nota '{}' inválida (deve estar entre 0 e 10).", valor);
                return true;
            }
            match memory.get_mut(&nome) {
                Some(student) => student.grades.push(valor),
                None => eprintln!("Erro: estudante '{}' não existe.", nome),
            }
        }
        Command::SHOW (nome) => {
            match memory.get(&nome) {
                Some(student) => {
                    println!("{}", student.name);
                    let notas: Vec<String> =
                        student.grades.iter().map(|n| format!("{:.1}", n)).collect();
                    println!("Notas: {}", notas.join(", "));
                }
                None => eprintln!("Erro: estudante '{}' não existe.", nome),
            }
        }
        Command::AVERAGE (nome) => {
            match memory.get(&nome) {
            Some(student) if !student.grades.is_empty() => {
                let media: f64 =
                    student.grades.iter().sum::<f64>() / student.grades.len() as f64;
                println!("Média: {:.2}", media);
            }
            Some(student) => println!("{} ainda não tem notas.", student.name),
            None => eprintln!("Erro: estudante '{}' não existe.", nome),
        }
        }
        Command::LIST => {
            let mut nomes: Vec<&String> = memory.keys().collect();
            nomes.sort();
            for nome in nomes {
                let student = &memory[nome];
                let notas: Vec<String> =
                    student.grades.iter().map(|n| format!("{:.1}", n)).collect();
                println!("{}: {}", nome, notas.join(", "));
            }
        }
        Command::QUIT => return false,
    }
    true
}