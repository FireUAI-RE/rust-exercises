pub struct Task{
  pub id: u32,
  description: String,
  done: bool
}

// métodos de task
impl Task{
  pub fn new(id_: u32, descrip: String, done_:bool) -> Task{
    Task {id: id_, description: descrip, done: done_}
  }

  pub fn show(&self){
    let check = match self.done{
      true => "X",
      false => " "
    };
    println!("[{}] [{}] {}", self.id, check, self.description)
  }

  pub fn done(&mut self){
    self.done = true;
  }

  pub fn info_as_string(&self) -> String{
    format!("{}|{}|{}\n", self.id, self.done, self.description)
  }
}

