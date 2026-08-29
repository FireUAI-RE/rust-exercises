
mod io_handling;
mod models;

use std::path::Path;

use crate::{io_handling::args::get_args, models::taks_vector::Tasks};
use models::actions::Action;
use io_handling::file_handler::{get_tasks, parse_to_file};

fn main() {
	let path = Path::new("tasks.txt");

	let action = match get_args(){
		Ok(a) => a,
		Err(e) => {
			println!("{}", e);
			return;
		}
	};

	let mut tasks = match  get_tasks(path){
		Err(e) =>{
			println!("{}", e);
			return;
		}
		Ok(ts) => Tasks{tasks: ts}
	};

	match action{
		Action::Add(description) =>{
			tasks.add(description);
		},

		Action::List => tasks.list(),

		Action::Show(id) =>{
			match tasks.show(id) {
					Ok(_) => (),
					Err(e) => println!("Erro: {}", e)
			}
		},

		Action::Done(id) =>{
			match tasks.done(id) {
					Ok(_) => (),
					Err(e) => println!("Erro: {}", e)
			}
		},

		Action::Remove(id) =>{
			match tasks.remove(id) {
					Ok(_) => (),
					Err(e) => println!("Erro: {}", e)
			}
		}
	}

	if let Err(e) = parse_to_file(path, tasks.tasks) { 
		println!("Error: {}", e)
	};
}
