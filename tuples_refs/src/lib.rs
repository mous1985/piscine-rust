#[derive(Debug,Clone)]
pub struct Student(i32, String, String);

pub fn id(student: &Student) -> i32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    let first=student.1.clone();
    first
}

pub fn last_name(student: &Student) -> String {
    let last=student.2.clone();
    last
}


