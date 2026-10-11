use std::process::exit;

use crate::{student};
use crate::student::Student;
use crate::utils::read_input;

pub enum Command{
  Add(String),
  Grade(String, String, f32, f32),
  Average(String),
  Show(String),
  Sum(String),
  List,
  Info,
  Quit
}

pub fn enum_parser() -> Result<Command, String>{
  let mut buffer = String::new();
  read_input(&mut buffer);

  let mut args: Vec<&str> = Vec::new();
  for arg in buffer.split_whitespace(){
    args.push(arg);
  }
  // fazer parser correto

  if args.is_empty(){
    return Err(String::from("Input at least one command!"));
  }

  match args[0] {
    "ADD" => {
      if args.len() != 2{
        return Err("Follow the patern: ADD <name>".to_string());
      } else{
        return Ok(Command::Add(args[1].to_string()));
      }
    },
    "GRADE" => {
      if args.len() != 5{
        return Err("Follow the patern: GRADE <name> <assignment_title> <score> <max_score>".to_string());
      } else{
        return Ok(
          Command::Grade(args[1].to_string(), 
          args[2].to_string(), 
          args[3].parse::<f32>().map_err(|e| e.to_string())?, // transforma o erro em tipo string para que possa ser usado no retorno
          args[4].parse::<f32>().map_err(|e| e.to_string())?)
        );
      }
    },
    "AVERAGE" => {
      if args.len() != 2{
        return Err("Follow the patern: AVERAGE <name>".to_string());
      } else{
        return Ok(Command::Average(args[1].to_string()));
      }
    },
    "SHOW" => {
      if args.len() != 2{
        return Err("Follow the patern: SHOW <name>".to_string());
      } else{
        return Ok(Command::Show(args[1].to_string()));
      }
    },
    "SUM" => {
      if args.len() != 2{
        return Err("Follow the patern: SUM <name>".to_string());
      } else{
        return Ok(Command::Sum(args[1].to_string()));
      }
    },
    "LIST" => {
      if args.len() != 1{
        return Err("Follow the patern: LIST".to_string());
      } else{
        return Ok(Command::List);
      }
    },
    "INFO" => {
      if args.len() != 1{
        return Err("Follow the patern: INFO".to_string());
      } else{
        return Ok(Command::Info);
      }
    },
    "QUIT" => {
      return Ok(Command::Quit);
    }
    _ => {
      return Err("Invalid input!".to_string());
    }
  }
}

pub fn print_commands(){
  println!("------------------------------");
  println!("Commands:\n");
  
  println!("ADD <name> -> add new student");
  println!("GRADE <name> <assignment_title> <score> <max_score> -> add new assignment");
  println!("AVERAGE <name> -> get student average");
  println!("SUM <name> -> get student's score sum");
  println!("LIST -> list all students");
  println!("INFO -> see commands");
  println!("QUIT -> quit program");
  println!("------------------------------");
}

pub fn command_execute(command: Command ,students_list: &mut Vec<Student>) ->Result<(), String>{
  match command{
    Command::Add(name) =>{
      match student::find_student(&name, students_list){
        Ok(_) => Err(format!("Student {} already registered!", name)),
        Err(e) if e.contains("not found") => {
          student::add_student(students_list, name);
          Ok(())
        }
        Err(e) => Err(e)
      }
    },
    Command::Grade(name, title,  score, max) => {
      let edit_student = student::find_student_mut(&name, students_list)?;
      edit_student.add_grade(&title, max, score)?;
      Ok(())
    },
    Command::Average(name) =>{
      let target_student = student::find_student(&name, students_list)?;
      let avg = target_student.get_avg();
      println!("{name}\nAverage: {avg}");
      Ok(())
    },
    Command::Show(name)=> {
      let target_student = student::find_student(&name, students_list)?;
      target_student.show();
      Ok(())
    }
    Command::Sum(name) =>{
      let target_student = student::find_student(&name, students_list)?;
      let (sum_scr, sum_max) = target_student.get_sum();
      println!("{name}\nGrades sum: {sum_scr}/{sum_max}");
      Ok(())
    },
    Command::List =>{
      student::list_students(students_list);
      Ok(())
    },
    Command::Info =>{
      print_commands();
      Ok(())
    },
    Command::Quit => {
      exit(0);
    }
  }
}