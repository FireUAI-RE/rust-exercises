pub struct Contato {
    nome: String,
    numero: String,
}

impl Contato {
    pub fn new(nome_: String, numero_: String) -> Self {
        Contato {
            nome: nome_,
            numero: numero_,
        }
    }

    pub fn get_nome(&self) -> String {
        self.nome.clone()
    }

    pub fn get_numero(&self) -> String {
        self.numero.clone()
    }
}
