pub struct Contador{
  pub nome: String,
  pub valor: i64
}

impl Contador{
  pub fn new(nome_: String, valor_: i64) -> Self{
    Contador{nome: nome_, valor: valor_}
  }

  pub fn set(&mut self, valor:i64){
    self.valor = valor;
  }

  pub fn add(&mut self, valor:i64){
    self.valor += valor;
  }

  pub fn sub(&mut self, valor:i64){
    self.valor -= valor;
  }

  pub fn print(&self){
    println!("{} = {}", self.nome, self.valor);
  }
}