// use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

struct Student {
    id: i64,
    name: String,
    grade: String,
}

struct StudentManagement {
    students: HashMap<i64, Student>,
}

impl StudentManagement {
    fn new() -> StudentManagement {
        StudentManagement {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        // self.students.entry(student.id).or_insert(student)yyp
        if self.students.contains_key(&student.id) {
            // Err("Student alredy in the collection".to_string())
            Err(format!("Student with ID {} already exists", student.id))
        } else {
            self.students.insert(student.id, student);
            Ok(())
        }
    }

    // fn add_student(&mut self, student: Student) -> Result<(), String> {
    // // self.students.entry(student.id).or_insert(student)yyp
    // match self.students.entry(student.id) {
    // Occupied(_) => Result::Err("Student alredy in the collection".to_string()),
    // Vacant(entry) => {
    // entry.insert(student);
    // Result::Ok(())
    // }
    // }
    // }

    fn get_student(&self, id: i64) -> Option<&Student> {
        self.students.get(&id)
    }
}

fn main() {
    let mut std_mng = StudentManagement::new();
    let student_1 = Student {
        id: 1,
        name: "Marcos".to_string(),
        grade: "Quardo ano".to_string(),
    };
    std_mng.add_student(student_1).unwrap();
    // let student_returned = std_mng.get_student(1).unwrap();
    if let Some(student_return) = std_mng.get_student(1) {};
}
