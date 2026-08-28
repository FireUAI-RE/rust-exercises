use super::contato::Contato;

pub struct Agenda {
    contatos: Vec<Contato>,
}

impl Agenda {
    pub fn new() -> Self {
        let contatos_: Vec<Contato> = Vec::new();
        Agenda {
            contatos: contatos_,
        }
    }

    // adiciona novo contato
    pub fn add_contato(&mut self, nome: &str, numero: &str) {
        let novo_contato = Contato::new(nome.to_string(), numero.to_string());
        self.contatos.push(novo_contato);
    }

    // pega o index do contato no vetor da agenda. Apenas para uso interno
    fn procura_index_contato(&self, nome: &str) -> Option<usize> {
        let mut i: u64 = 0; 
        for contato in &self.contatos {
            if contato.get_nome().to_lowercase() == nome.to_lowercase() {
                return Some(i as usize);
            }
            i += 1;
        }
        None
    }

    pub fn procura_contato(&self, nome: &str) -> Result<&Contato, String> {
        match self.procura_index_contato(nome) {
            Some(i) => Ok(&self.contatos[i]),
            None => Err("contato não encontrado!".to_string()),
        }
    }

    pub fn remove_contato(&mut self, nome: &str) -> Result<(), String> {
        match self.procura_index_contato(nome) {
            Some(i) => {
                self.contatos.remove(i);
                Ok(())
            }
            None => Err("contato não encontrado!".to_string()),
        }
    }

    pub fn lista_contatos(&self) {
        if self.contatos.is_empty() {
            print!("Não há contatos registrados!");
            return;
        }

        for contato in &self.contatos {
            println!("{}: {}", contato.get_nome(), contato.get_numero());
        }
    }
}
