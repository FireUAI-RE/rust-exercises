use super::task::Task;

pub struct Tasks{
  pub tasks: Vec<Task>
}

impl Tasks{
  fn find_task_index(&self, id: u32) -> Option<usize> {
    let mut i: usize = 0;

    while i < self.tasks.len() {
        if self.tasks[i].id == id {
            return Some(i);
        }
        i += 1;
    }
    None
}

  pub fn add(&mut self, descrip: String){
    let new_id = match self.tasks.last(){
      Some(x) => x.id+1,
      None => 0
    };
    let new = Task::new(new_id, descrip, false);
    self.tasks.push(new);
  }

  pub fn list(&self){
    if self.tasks.is_empty() {
      println!("Sem tarefas!");
      return;
    }

    for t in &self.tasks{
      t.show();
    }
  }

  pub fn show(&self, id: u32)-> Result<(), String>{
    match self.find_task_index(id){
      Some(index) => {
        self.tasks[index].show();
      },
      None => { return Err("Insira um id válido!".to_string());} 
    }
    Ok(())
  }

  pub fn done(&mut self, id: u32)-> Result<(), String>{
    match self.find_task_index(id){
      Some(index) => {
        self.tasks[index].done();
      },
      None => { return Err("Insira um id válido!".to_string());} 
    }
    Ok(())
  }

  pub fn remove(&mut self, id: u32)-> Result<(), String>{
    match self.find_task_index(id){
      Some(x) => {
        self.tasks.remove(x);
      },
      None => { return Err("Insira um id válido!".to_string());} 
    }
    Ok(())
  }
}