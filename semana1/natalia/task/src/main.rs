use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const CAMINHO_ARQUIVO: &str = "tarefas.txt";

#[derive(Clone)]
struct Tarefa {
    id: usize,
    concluida: bool,
    descricao: String,
}

struct GerenciadorTarefas {
    tarefas: Vec<Tarefa>,
    proximo_id: usize,
}

impl GerenciadorTarefas {
    fn new() -> Self {
        Self {
            tarefas: Vec::new(),
            proximo_id: 0,
        }
    }

    fn adicionar(&mut self, descricao: String) {
        let tarefa = Tarefa {
            id: self.proximo_id,
            concluida: false,
            descricao,
        };
        self.tarefas.push(tarefa);
        self.proximo_id += 1;
    }

    fn concluir(&mut self, id: usize) -> Result<(), String> {
        if let Some(tarefa) = self.tarefas.iter_mut().find(|t| t.id == id) {
            tarefa.concluida = true;
            Ok(())
        } else {
            Err(format!("tarefa com ID {} não encontrada.", id))
        }
    }

    fn remover(&mut self, id: usize) -> Result<(), String> {
        let initial_len = self.tarefas.len();
        self.tarefas.retain(|t| t.id != id);
        if self.tarefas.len() < initial_len {
            Ok(())
        } else {
            Err(format!("tarefa com ID {} não encontrada.", id))
        }
    }

    fn obter_tarefa(&self, id: usize) -> Option<&Tarefa> {
        self.tarefas.iter().find(|t| t.id == id)
    }
}

fn carregar_tarefas(caminho: &str) -> GerenciadorTarefas {
    let mut gerenciador = GerenciadorTarefas::new();

    if !Path::new(caminho).exists() {
        return gerenciador;
    }

    let arquivo = match File::open(caminho) {
        Ok(f) => f,
        Err(_) => return gerenciador,
    };

    let leitor = BufReader::new(arquivo);
    let mut id_maximo = 0;

    for resultado_linha in leitor.lines() {
        if let Ok(linha) = resultado_linha {
            let partes: Vec<&str> = linha.splitn(3, '|').collect();
            if partes.len() == 3 {
                if let (Ok(id), Ok(concluida)) =
                    (partes[0].parse::<usize>(), partes[1].parse::<bool>())
                {
                    gerenciador.tarefas.push(Tarefa {
                        id,
                        concluida,
                        descricao: partes[2].to_string(),
                    });
                    if id >= id_maximo {
                        id_maximo = id + 1;
                    }
                }
            }
        }
    }

    gerenciador.proximo_id = id_maximo;
    gerenciador
}

fn salvar_tarefas(gerenciador: &GerenciadorTarefas, caminho: &str) -> Result<(), String> {
    let mut arquivo = File::create(caminho).map_err(|e| format!("erro ao criar arquivo: {}", e))?;

    for tarefa in &gerenciador.tarefas {
        let linha = format!("{}|{}|{}\n", tarefa.id, tarefa.concluida, tarefa.descricao);
        arquivo
            .write_all(linha.as_bytes())
            .map_err(|e| format!("erro ao escrever no arquivo: {}", e))?;
    }
    Ok(())
}

fn print_tarefa(tarefa: &Tarefa) {
    let mark = if tarefa.concluida { "x" } else { " " };
    println!("[{}] [{}] {}", tarefa.id, mark, tarefa.descricao);
}

fn print_lista(gerenciador: &GerenciadorTarefas) {
    if gerenciador.tarefas.is_empty() {
        println!("nenhuma tarefa cadastrada.");
    } else {
        for tarefa in &gerenciador.tarefas {
            print_tarefa(tarefa);
        }
    }
}

fn executar_cli() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("comando não encontrado.".to_string());
    }

    let comando = args[1].as_str();
    let mut gerenciador = carregar_tarefas(CAMINHO_ARQUIVO);

    match comando {
        "add" => {
            if args.len() < 3 {
                return Err("falta a descrição da tarefa.".to_string());
            }
            let descricao = args[2..].join(" ");
            gerenciador.adicionar(descricao);
            salvar_tarefas(&gerenciador, CAMINHO_ARQUIVO)?;
            println!("tarefa adicionada");
        }
        "list" => {
            print_lista(&gerenciador);
        }
        "show" => {
            if args.len() < 3 {
                return Err("falta o ID da tarefa".to_string());
            }
            let id: usize = args[2]
                .parse()
                .map_err(|_| "ID deve ser um número válido.")?;
            if let Some(tarefa) = gerenciador.obter_tarefa(id) {
                print_tarefa(tarefa);
            } else {
                return Err(format!("tarefa com ID {} não encontrada.", id));
            }
        }
        "done" => {
            if args.len() < 3 {
                return Err("Falta o ID da tarefa".to_string());
            }
            let id: usize = args[2]
                .parse()
                .map_err(|_| "ID deve ser um número válido.")?;
            gerenciador.concluir(id)?;
            salvar_tarefas(&gerenciador, CAMINHO_ARQUIVO)?;
            println!("tarefa {} concluída!", id);
        }
        "remove" => {
            if args.len() < 3 {
                return Err("falta o ID da tarefa.".to_string());
            }
            let id: usize = args[2]
                .parse()
                .map_err(|_| "ID deve ser um número válido.")?;
            gerenciador.remover(id)?;
            salvar_tarefas(&gerenciador, CAMINHO_ARQUIVO)?;
            println!("tarefa {} removida!", id);
        }
        _ => {
            return Err(format!("comando '{}' desconhecido", comando));
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = executar_cli() {
        eprintln!("erro: {}", e);
    }
}
