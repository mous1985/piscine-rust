#[derive(Debug,Clone)]
pub struct Student(i32, String, String);

pub fn id(student: &Student) -> i32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.clone()
}

pub fn last_name(student: &Student) -> String {
    student.2.clone()
}


