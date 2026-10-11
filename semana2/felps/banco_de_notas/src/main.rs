use crate::{
    commands::{command_execute, enum_parser},
    student::Student,
};

mod assignment;
mod commands;
mod student;
mod utils;
fn main() {
    commands::print_commands();
    let mut students: Vec<Student> = Vec::new();
    loop {
        let action = match enum_parser() {
            Ok(c) => c,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        match command_execute(action, &mut students) {
            Ok(()) => (),
            Err(e) => {
                println!("{e}");
                continue;
            }
        }
        println!();
    }
}
