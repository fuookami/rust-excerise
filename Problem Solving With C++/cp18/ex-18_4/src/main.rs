#[derive(Clone)]
struct StudentInfo {
    name: String,
    grade: i64,
}

fn select_failed_students(array: &Vec<StudentInfo>) -> Vec<StudentInfo> {
    array
        .iter()
        .filter(|student| student.grade < 60)
        .cloned()
        .collect()
}

fn select_succeed_students(array: &Vec<StudentInfo>) -> Vec<StudentInfo> {
    array
        .iter()
        .filter(|student| student.grade >= 60)
        .cloned()
        .collect()
}

fn main() {
    println!("Hello, world!");
}
