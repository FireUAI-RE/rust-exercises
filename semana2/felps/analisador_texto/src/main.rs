use std::io;
use std::io::Write;

fn main() {
	// read user input
	let mut buffer = String::new();
	read_input(&mut buffer);

	print!("Palavra mais procurada: ");
	io::stdout().flush().unwrap(); // clear stdout, makes so prints before second input  limpa stdout, faz com que o print seja impresso na hora (estava sendo impresso so depois de receber o 2° input)

	let mut target_word = String::new();
	read_input(&mut target_word);
	
	let total_chars: usize = buffer.chars().count(); // char count

	let (word_counter, target_word_c, biggest_word) = info_words(&buffer, &target_word); // word count

	println!("caracteres: {total_chars}\nPalavras: {word_counter}\nMaior palavra: {biggest_word}\nOcorrencias de {target_word}: {target_word_c}");
}

fn info_words(input: &str, target_word: &str) -> (u32, u32, String ){
	let mut word_c = 0;
	let mut target_word_c: u32 = 0;
	let mut biggest_word = "";

	let mut bw_size: usize = biggest_word.len();

	for word in input.split_whitespace(){
		word_c +=1; // add in word counter
		// compare biggest word
		if bw_size < word.len(){ 
			bw_size = word.len();
			biggest_word = word;
		}
		// compare to user specific word 
		if word.to_lowercase() == target_word.to_lowercase(){ 
			target_word_c +=1;
		}
	}

	return (word_c, target_word_c, biggest_word.to_string());
}

fn read_input(buffer:&mut String){
	loop{
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