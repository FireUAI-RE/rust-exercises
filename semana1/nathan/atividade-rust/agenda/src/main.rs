use std::io;

struct Contact {
    name: String,
    number: String,
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let arguments: Vec<String> = input
            .split_whitespace()
            .filter_map(|s| s.parse::<String>().ok())
            .collect();

        if arguments.is_empty() {
            println!("Nenhum número válido foi fornecido.");
            return;
        }

        let command: &String = &arguments[0];
        
        if command == "QUIT" {
            break;
        }

        if arguments.len() >= 2 {
            let name: &String = &arguments[1];
            if command == "ADD" {
                let number: &String = &arguments[2];
                contacts.push(Contact{name: name.to_string(), number: number.to_string()});
            } else if command == "GET" {
                for contact in &contacts {
                    if contact.name == *name {
                        println!("Name: {}, Phone: {}", contact.name, contact.number);
                    }
                }
            } else if command == "REMOVE" {
                contacts.retain(|contact| contact.name != *name);
            }
        } else {
            if command == "LIST" {
                println!("===== The Contacts List =====");
                for contact in &contacts {
                    println!("Name: {}, Phone: {}", contact.name, contact.number);
                }
            } else {
                println!("Invalid Command");
            }
        }
    } 
}
