use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::path::Path;
use std::io::ErrorKind;

use crate::models::task::Task;

pub fn get_tasks(path: &Path) -> Result<Vec<Task>, String>{
  // abrindo o arquivo
  let display = path.display();

  match File::open(path) {
    Err(error) => match error.kind() {
      // se n tem arquivo, cria ele e retorna vetor vazio
      ErrorKind::NotFound => Ok(Vec::new()),
      _ => Err(format!("Cannot open file {}!", display)),
    },
    // transforma em vetor e retorna esse vetor
    Ok(file) => parse_to_task_vec(file)
  }
}

fn parse_to_task_vec(mut file: File) -> Result<Vec<Task>, String>{
  let mut s = String::new();
  let mut tasks: Vec<Task> = Vec::new();
  match file.read_to_string(&mut s) {
      Err(_) => {return Err(String::from("Unable to open file"));},
      Ok(_) => {
        for line in s.lines(){
          let split_task: Vec<&str> = line.split('|').collect();

          if split_task.len() != 3 {return Err("File contains a partial task".to_string());}
          let id = split_task[0].parse::<u32>().map_err(|_| "File contains invalid ID format".to_string())?; // faz com que o erro possa ser passado com "?"
          // pq isso normalmente n ertorna um Err(String)
          let done = match split_task[1]{
            "true" => true,
            "false" => false,
            _ => { return Err("File contains wrong data".to_string());} 
          };
          let description = split_task[2];

          tasks.push(Task::new(id, description.to_string(), done));
        };
      }
  }

  Ok(tasks)
}

fn create_file(path: &Path) -> Result<File, String>{
  match File::create(path) {
    Err(_) => Err("Couldn't create file".to_string()),
    Ok(file) => Ok(file)
  }
}

pub fn parse_to_file(path: &Path, tasks: Vec<Task>) -> Result<(), String>{
  let mut file = create_file(path)?;
  for task in tasks{
    if file.write_all(task.info_as_string().as_bytes()).is_err(){
      // recomendação do clippy
      return Err("Couldn't write to file".to_string());
    }
  }
  Ok(())
}