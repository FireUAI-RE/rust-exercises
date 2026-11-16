pub struct Assignment {
    pub title: String,
    pub max_score: f32,
    pub score: f32,
}

impl Assignment {
    pub fn print(&self) {
        println!("{}: {}/{}", self.title, self.score, self.max_score);
    }
}
