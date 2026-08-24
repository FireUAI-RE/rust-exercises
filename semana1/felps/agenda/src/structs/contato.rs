pub struct Contato {
  nome: String,
  numero: String
}

impl Contato {
  pub fn new(nome: String, numero: String) -> Self {
    Contato { nome: nome, numero: numero }
  }

  pub fn get_nome(&self) -> String{
    self.nome.clone()
  }

  pub fn get_numero(&self) -> String{
    self.numero.clone()
  }
}
