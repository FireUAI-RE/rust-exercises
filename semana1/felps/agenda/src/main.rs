mod structs;
mod utils;

use std::process::exit;

use structs::agenda::Agenda;
// use structs::contato::Contato;

use utils::input::*;

fn main() {
	print_bem_vindo();

	let mut agenda= Agenda::new();
	
	loop{
		let comandos = escolher_comando();

		let informatado_msg = "Siga o formato apresentado!";

		match comandos[0].as_str() { // .as_str pq NÃO DEIXAM eu pegar ownership e o borrow simplesmente não funcionou
    	"ADD" => {
				if comandos.len() != 3{
					println!("{}", informatado_msg);
					continue;
				} 
				
				agenda.add_contato(comandos[1].as_str(), comandos[2].as_str());
			},
			"GET" => {
				if comandos.len() != 2{
					println!("{}", informatado_msg);
					continue;
				} 

				match agenda.procura_contato(comandos[1].as_str()){
					Ok(contato) => println!("{}: {}", contato.get_nome(), contato.get_numero()),
					Err(e) => println!("Erro: {}", e)
				}
			}
			"REMOVE" => {
				if comandos.len() != 2{
					println!("{}", informatado_msg);
					continue;
				} 

				match agenda.remove_contato(comandos[1].as_str()){
					Ok(_) => println!("{} removido", comandos[1]),
					Err(e) => println!("Erro: {}", e)
				}
			}
			"LIST" => {
				if comandos.len() != 1{
					println!("{}", informatado_msg);
					continue;
				} 

				agenda.lista_contatos();
			}
			"QUIT" => exit(1),
  	  _ => println!("Comando inválido!"),
	}
		
	}






}
