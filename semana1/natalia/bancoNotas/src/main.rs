#![allow(non_snake_case)] //pq sim :)

use std::io;

struct Estudante {
    nome: String,
    notas: Vec<f32>,
}

impl Estudante {
    fn media(&self) -> Option<f32> {
        if self.notas.is_empty() {
            return None;
        }
        let soma: f32 = self.notas.iter().sum();
        Some(soma / self.notas.len() as f32)
    }
}

struct Turma {
    estudantes: Vec<Estudante>,
}

impl Turma {
    fn adicionarEstudante(&mut self, nome: String) {
        self.estudantes.push(Estudante { nome, notas: Vec::new() });
    }

    fn buscar(&self, nome: &str) -> Option<&Estudante> {
        self.estudantes.iter().find(|e| e.nome == nome)
    }

    fn adicionarNota(&mut self, nome: &str, nota: f32) -> bool {
        match self.estudantes.iter_mut().find(|e| e.nome == nome) {
            Some(estudante) => {
                estudante.notas.push(nota);
                true
            }
            None => false,
        }
    }
}

fn lerNota(texto: &str) -> Option<f32> {
    match texto.parse::<f32>() {
        Ok(nota) if nota >= 0.0 && nota <= 10.0 => Some(nota),
        _ => None,
    }
}

fn main() {
    let mut turma = Turma { estudantes: Vec::new() };

    for linha in io::stdin().lines() {
        let linha = match linha {
            Ok(linha) => linha,
            Err(_) => break,
        };

        let mut partes = linha.split_whitespace();
        let comando = partes.next().unwrap_or("");
        let nome = partes.next().unwrap_or("");
        let valor = partes.next().unwrap_or("");

        match comando {

            "ADD" => turma.adicionarEstudante(nome.to_string()),
            "GRADE" => match lerNota(valor) {
                Some(nota) => {
                    if !turma.adicionarNota(nome, nota) {
                        println!("estudante {} não encontrado", nome);
                    }
                }
                None => println!("nota invalida: {}", valor),
            },
            "SHOW" => match turma.buscar(nome) {
                Some(estudante) => {
                    println!("{}", estudante.nome);
                    println!("notas: {:?}", estudante.notas);
                }
                None => println!("estudante {} não encontrado", nome),
            },
            "AVERAGE" => match turma.buscar(nome) {
                Some(estudante) => match estudante.media() {
                    Some(media) => println!("média: {:.2}", media),
                    None => println!("{} não tem notas", nome),
                },
                None => println!("estudante {} não encontrado", nome),
            },
            "LIST" => {
                for estudante in &turma.estudantes {
                    println!("{}: {:?}", estudante.nome, estudante.notas);
                }
            }
            "" => {} //linha em branco
            _ => println!("comando desconhecido: {}", comando),
        }
    }
}
