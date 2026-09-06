use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Contato {
    nome: String,
    telefone: String,
}

struct Agenda {
    tabela_contatos: HashMap<String, Contato>,
}

impl Agenda {
    fn criar_nova() -> Self {
        Agenda {
            tabela_contatos: HashMap::new(),
        }
    }

    fn adicionar_contato(&mut self, nome: String, telefone: String) {
        let novo_contato = Contato {
            nome: nome.clone(),
            telefone,
        };
        self.tabela_contatos.insert(nome.clone(), novo_contato);
        println!("{} adicionado", nome);
    }

    fn buscar_contato(&self, nome: &str) {
        match self.tabela_contatos.get(nome) {
            Some(contato) => println!("{}: {}", contato.nome, contato.telefone),
            None => println!("contato '{}' não encontrado", nome),
        }
    }

    fn remover_contato(&mut self, nome: &str) {
        if self.tabela_contatos.remove(nome).is_some() {
            println!("{} removido", nome);
        } else {
            println!("contato '{}' não encontrado", nome);
        }
    }

    fn listar_todos(&self) {
        if self.tabela_contatos.is_empty() {
            println!("agenda vazia");
            return;
        }

        for contato in self.tabela_contatos.values() {
            println!("{}: {}", contato.nome, contato.telefone);
        }
    }
}

fn main() {
    let mut agenda = Agenda::criar_nova();
    let entrada = io::stdin();

    loop {
        let mut linha_digitada = String::new();

        if entrada.read_line(&mut linha_digitada).unwrap_or(0) == 0 {
            break;
        }

        let palavras: Vec<&str> = linha_digitada.trim().split_whitespace().collect();

        if palavras.is_empty() {
            continue;
        }

        let comando = palavras[0].to_uppercase();

        match comando.as_str() {
            "ADD" => {
                if palavras.len() >= 3 {
                    let nome = palavras[1].to_string();
                    let telefone = palavras[2].to_string();
                    agenda.adicionar_contato(nome, telefone);
                } else {
                    println!(" ADD <nome> <telefone>");
                }
            }
            "GET" => {
                if palavras.len() >= 2 {
                    let nome = palavras[1];
                    agenda.buscar_contato(nome);
                } else {
                    println!(" GET <nome>");
                }
            }
            "REMOVE" => {
                if palavras.len() >= 2 {
                    let nome = palavras[1];
                    agenda.remover_contato(nome);
                } else {
                    println!(" REMOVE <nome>");
                }
            }
            "LIST" => {
                agenda.listar_todos();
            }
            "QUIT" => {
                break;
            }
            _ => {
                println!("comando errado: {}", palavras[0]);
            }
        }
    }
}
