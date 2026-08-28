pub enum Comando {
    Set((String, i64)),
    Add((String, i64)),
    Sub((String, i64)),
    Print(Option<String>),
    Quit,
}
