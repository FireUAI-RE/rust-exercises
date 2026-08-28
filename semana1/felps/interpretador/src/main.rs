mod enums;
mod utils;

use utils::input;
use enums::comands::Comando;
use enums::contador::Contador;

use enums::vetor_contadores::*;



fn main() {
  let mut contadores :Vec<Contador> = Vec::new();
  loop{
    let comando = match input::pega_comando(){
      Err(e) => {
        println!("Erro: {}", e);
        continue;
      },
      Ok(comando_enum) => comando_enum
    };  

    match comando{
      Comando::Add((nome, valor)) => {
        match procura_contador(&mut contadores, &nome){
          Some(c) => c.add(valor),
          None => println!("Erro: não foi encontrado o contador {}", nome)
        };
      },

      Comando::Set((nome, valor)) => {
        match procura_contador(&mut contadores, &nome){
          Some(c) => c.set(valor),
          None => contadores.push(Contador::new(nome, valor))
        };
      },

      Comando::Sub((nome, valor)) => {
        match procura_contador(&mut contadores, &nome){
          Some(c) => c.sub(valor),
          None => println!("Erro: não foi encontrado o contador {}", nome)
        };
      },

      Comando::Print(nome) => {
        match nome {
          None => print_all(&contadores),
          Some(nome) => {
            match procura_contador(&mut contadores, &nome){
              Some(c) => c.print(),
              None => println!("Erro: não foi encontrado o contador {}", nome)
            };
          }
        }
      },

      Comando::Quit => return
    }
  }
}
