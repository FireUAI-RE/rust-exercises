use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write};

static FILE_PATH:&str = "src/tasks.txt";

struct Task {
    index: u128,
    done: bool,
    name: String,
}

enum Command {
    Add,
    List,
    Done,
    Remove,
    Show,
}

fn file_manipulator(command: Command, action: Option<&String>) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(FILE_PATH)?;

    let total_lines = BufReader::new(&file).lines().count();

    match command {
        Command::Add => {
            let task = Task {
                index: (total_lines + 1) as u128,
                done: false,
                name: action.cloned().unwrap_or_default(),
            };

            let mut file_write = OpenOptions::new().append(true).open(FILE_PATH)?;
            writeln!(file_write, "{} | {} | {}", task.index, task.done, task.name)?;
        },
        Command::Done => {
            if let Some(target_index) = action {
                let file = fs::read_to_string(FILE_PATH)?;
                let mut new_lines = Vec::new();
                for line in file.lines() {
                    let parts: Vec<&str> = line.split(" | ").collect();

                    if parts.len() == 3 && parts[0] == target_index {
                        new_lines.push(format!("{} | true | {}", parts[0], parts[2]));
                    } else {
                        new_lines.push(line.to_string());
                    }
                }
                fs::write(FILE_PATH, new_lines.join("\n") + "\n")?;
            }
        },
        Command::List => {
            let file = File::open(FILE_PATH)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.split(" | ").collect();

                if parts.len() == 3 {
                    let index = parts[0];
                    let done = if parts[1] == "true" { "[X]" } else { "[ ]" };
                    let name = parts[2];
                    println!("{} | {} | {}", index, done, name);
                }
            }
        },
        Command::Remove => {
            if let Some(target_index) = action {
                let file = fs::read_to_string(FILE_PATH)?;
                let mut new_lines = Vec::new();
                for line in file.lines() {
                    let parts: Vec<&str> = line.split(" | ").collect();

                    if parts.len() == 3 && parts[0] == target_index {
                        continue;
                    } else {
                        new_lines.push(line.to_string());
                    }
                }
                fs::write(FILE_PATH, new_lines.join("\n") + "\n")?;
            }
        },
        Command::Show => {
            if let Some(target_index) = action {
                let file = File::open(FILE_PATH)?;
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let line = line?;
                    let parts: Vec<&str> = line.split(" | ").collect();

                    if parts.len() == 3 && parts[0] == target_index {
                        let index = parts[0];
                        let done = if parts[1] == "true" { "[X]" } else { "[ ]" };
                        let name = parts[2];
                        println!("{} | {} | {}", index, done, name);
                        break;
                    }
                }
            }
        },
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: cargo run -- <comando> <acao>");
        return;
    }

    let command = match args[1].trim().to_uppercase().as_str() {
        "ADD" => Command::Add,
        "LIST" => Command::List,
        "DONE" => Command::Done,
        "REMOVE" => Command::Remove,
        "SHOW" => Command::Show,
        _ => return,
    };

    let action = args.get(2);

    let _ = file_manipulator(command, action);
}
