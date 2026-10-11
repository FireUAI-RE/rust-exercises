use crate::assignment::Assignment;


pub struct Student {
  name: String,
  grades: Vec<Assignment>,
}

impl Student {
  pub fn show(&self){
    println!("{}", self.name);
    for grade in self.grades.iter(){
      grade.print();
    }
  }

  pub fn add_grade(&mut self, ass_title: &str, new_grade_max: f32, new_grade: f32) -> Result<(), String>{
    if new_grade < 0.0 {
      return Err(String::from("Grade cant be negative!"));
    }
    
    if new_grade > new_grade_max {
      return Err(String::from("Grade cant be greater than its max!"));
    }

    let new_ass = Assignment{ 
      title: ass_title.to_string(), 
      max_score: new_grade_max, 
      score: new_grade
    };

    self.grades.push(new_ass);
    Ok(())
  }

  pub fn get_sum(&self) -> (f32, f32){
    let mut sum_scr: f32 = 0.0;
    let mut sum_max: f32 = 0.0;
    for grade in self.grades.iter(){
      sum_scr += grade.score;
      sum_max += grade.max_score;
    }
    return (sum_scr, sum_max);
  }

  pub fn get_avg(&self) -> f32{
    match self.get_sum().0{
      0.0 => 0.0,
      num => num/self.grades.len() as f32
    }
  }
}

pub fn find_student_mut<'a>(name: &str, student_list: &'a mut Vec<Student>) -> Result<&'a mut Student, String>{
  let name = name.to_lowercase();
  for student in student_list.iter_mut(){
    if student.name.to_lowercase() == name{
      return Ok(student);
    }
  }
  Err(format!("Student {name} not found!"))
}

pub fn find_student<'a>(name: &str, student_list: &'a Vec<Student>) -> Result<&'a Student, String>{
  let name = name.to_lowercase();
  for student in student_list.iter(){
    if student.name.to_lowercase() == name{
      return Ok(student);
    }
  }
  Err(format!("Student {name} not found!"))
}

pub fn add_student(student_list: &mut Vec<Student>, name: String){
  let new_std = Student{
    name: name,
    grades: Vec::new()
  };

  student_list.push(new_std);
}

pub fn list_students(student_list: &Vec<Student>){
  if student_list.is_empty(){
    println!("NO STUDENTS REGISTERED!");
    return;
  }
  println!("STUDENTS:");
  for student in student_list.iter(){
    println!("{}", student.name);
  }
}