use std::io;
use std::collections::HashMap;

struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: String) -> Self {
        Student { name, grades: Vec::new() }
    }

    fn add_grade(&mut self, valor: f64) -> Result<(), String> {
        if !(0.0..=10.0).contains(&valor) {
            return Err(format!("nota '{}' inválida (deve estar entre 0 e 10).", valor));
        }
        self.grades.push(valor);
        Ok(())
    }

    fn average(&self) -> Option<f64> {
        if self.grades.is_empty() {
            None
        } else {
            Some(self.grades.iter().sum::<f64>() / self.grades.len() as f64)
        }
    }

    fn grades_str(&self) -> String {
        self.grades.iter().map(|n| format!("{:.1}", n)).collect::<Vec<_>>().join(", ")
    }
}

enum Command<'a> {
    Add(&'a str),
    Grade { nome: &'a str, valor: f64 },
    Show(&'a str),
    Average(&'a str),
    List,
    Quit,
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

        if let Some(cmd) = interpretator(&input) {
            if !exec_commands(cmd, &mut memory) { break; }
        }
    }
}

fn interpretator(linha: &str) -> Option<Command<'_>> {
    let mut partes = linha.split_whitespace();
    let cmd = partes.next()?.to_uppercase();

    match cmd.as_str() {
        "ADD" | "SHOW" | "AVERAGE" => {
            let Some(nome) = partes.next() else {
                eprintln!("Erro: O comando '{}' precisa de nome.", cmd);
                return None;
            };
            Some(match cmd.as_str() {
                "ADD" => Command::Add(nome),
                "SHOW" => Command::Show(nome),
                _ => Command::Average(nome),
            })
        }
        "GRADE" => {
            let (Some(nome), Some(v)) = (partes.next(), partes.next()) else {
                eprintln!("Erro: O comando '{}' precisa de nome e valor.", cmd);
                return None;
            };
            match v.parse::<f64>() {
                Ok(valor) => Some(Command::Grade { nome, valor }),
                Err(_) => {
                    eprintln!("Erro: '{}' não é um número válido.", v);
                    None
                }
            }
        }
        "LIST" => Some(Command::List),
        "QUIT" => Some(Command::Quit),
        _ => {
            eprintln!("Erro: Comando não reconhecido.");
            None
        }
    }
}

fn exec_commands(cmd: Command, memory: &mut HashMap<String, Student>) -> bool {
    match cmd {
        Command::Add(nome) => add_student(memory, nome),
        Command::Grade { nome, valor } => add_grade(memory, nome, valor),
        Command::Show(nome) => show_student(memory, nome),
        Command::Average(nome) => show_average(memory, nome),
        Command::List => list_students(memory),
        Command::Quit => return false,
    }
    true
}

fn add_student(memory: &mut HashMap<String, Student>, nome: &str) {
    if memory.contains_key(nome) {
        eprintln!("Erro: estudante '{}' já existe.", nome);
        return;
    }
    memory.insert(nome.to_string(), Student::new(nome.to_string()));
}

fn add_grade(memory: &mut HashMap<String, Student>, nome: &str, valor: f64) {
    match memory.get_mut(nome) {
        Some(student) => {
            if let Err(e) = student.add_grade(valor) {
                eprintln!("Erro: {}", e);
            }
        }
        None => eprintln!("Erro: estudante '{}' não existe.", nome),
    }
}

fn find_student<'a>(memory: &'a HashMap<String, Student>, nome: &str) -> Option<&'a Student> {
    let student = memory.get(nome);
    if student.is_none() {
        eprintln!("Erro: estudante '{}' não existe.", nome);
    }
    student
}

fn show_student(memory: &HashMap<String, Student>, nome: &str) {
    if let Some(student) = find_student(memory, nome) {
        println!("{}", student.name);
        println!("Notas: {}", student.grades_str());
    }
}

fn show_average(memory: &HashMap<String, Student>, nome: &str) {
    if let Some(student) = find_student(memory, nome) {
        match student.average() {
            Some(media) => println!("Média: {:.2}", media),
            None => println!("{} ainda não tem notas.", student.name),
        }
    }
}

fn list_students(memory: &HashMap<String, Student>) {
    let mut nomes: Vec<&String> = memory.keys().collect();
    nomes.sort();
    for nome in nomes {
        println!("{}: {}", nome, memory[nome].grades_str());
    }
}