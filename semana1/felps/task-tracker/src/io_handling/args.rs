use std::env;
use crate::models::actions::Action;

pub fn get_args() -> Result<Action, String>{
  let args: Vec<String> = env::args().collect();
    // primeiro argumento é o caminho do programa
    if args.len() <= 1{
      return Err(String::from("Provide at least one argument!"));
    }

    match args[1].to_lowercase().as_str() {
      "add" => {
        if args.len() != 3{
          Err(String::from("Follow the format \"add <descricao>\""))
        } else{
          Ok(Action::Add(args[2].clone()))
        }
      }

      "list" => {
        if args.len() != 2{
          Err(String::from("Do not provide extra arguments! Faça: \"list\""))
        } else{
          Ok(Action::List)
        }
      }

      "show" => {
        if args.len() != 3{
          Err(String::from("Follow the format \"show <id>\""))
        } else{
          let id = parse_id(args[2].as_str())?;
          Ok(Action::Show(id))
        }
      }

      "done" => {
        if args.len() != 3{
          Err(String::from("Follow the format \"done <id>\""))
        } else{
          let id = parse_id(args[2].as_str())?;
          Ok(Action::Done(id))
        }
      }

      "remove" => {
        if args.len() != 3{
          Err(String::from("Follow the format \"remove <id>\""))
        } else{
          let id = parse_id(args[2].as_str())?;
          Ok(Action::Remove(id))
        }
      }

      _ => Err(String::from("Invalid command!"))
    }
}

fn parse_id(num: &str)-> Result<u32, String> {
  match num.parse::<u32>() {
    Ok(n) => Ok(n),
    Err(_) => Err("Invalid ID format!".to_string())
  }
}