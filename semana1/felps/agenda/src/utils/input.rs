use std::io;

pub fn print_bem_vindo(){
	println!("\nBem vindo a agenda digital!");
}

fn print_separador(){
	println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=");
}

pub fn print_opcoes(){
	println!("");
	print_separador();
	println!("    Digite um comando:");
	print_separador();
    println!("\n\"ADD [nome] [numero]\" -> adiciona um contato
\"GET [nome]\" -> pega info de um contato
\"REMOVE [nome]\" -> remove um contato
\"LIST\" -> lista todos os contatos
\"QUIT\" -> encerra o programa\n");
}

fn trata_input(input: &String) -> Result<Vec<String>, String>{
	let mut params: Vec<String> = Vec::new();
	
// separacao do input em partes
	for elem in input.split_whitespace(){
		params.push(elem.to_string());
	}

	if params.len() <= 0{
		return Err(String::from("Insira 1 comando!"));
	}
	return Ok(params);
}


pub fn escolher_comando() -> Vec<String>{
	loop{
		print_opcoes();
	
		let mut buffer = String::new();
		match io::stdin().read_line(&mut buffer){
			Ok(_) => (),
			Err(e)=>{
				println!("Erro ao receber input: {}\nTente novamente", e);
			} 
		};

		match trata_input(&buffer){
			Ok(coms) => return coms,
			Err(e) => {
				println!("Erro: {}", e);
				continue;
			}
		};
	}
}