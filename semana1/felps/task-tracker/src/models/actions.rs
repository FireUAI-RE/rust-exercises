#[derive(Debug)]
pub enum Action{
  Add(String),
  List,
  Show(u32),
  Done(u32),
  Remove(u32)
}