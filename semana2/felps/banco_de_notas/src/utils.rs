use std::io;

pub fn read_input(buffer: &mut String) {
    loop {
        match io::stdin().read_line(buffer) {
            Ok(_) => {
                *buffer = buffer.trim().to_string();
                break;
            }
            Err(e) => {
                println!("Erro ao receber input: {e}\nTente novamente");
            }
        }
    }
}
