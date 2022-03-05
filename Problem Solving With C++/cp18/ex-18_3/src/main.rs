struct StudentInfo {
    name: String,
    grade: i64,
}

fn sort(array: &mut Vec<StudentInfo>) {
    array.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name))
}

fn min_grade(array: &Vec<StudentInfo>) -> i64 {
    array
        .iter()
        .min_by(|lhs, rhs| lhs.grade.cmp(&rhs.grade))
        .unwrap()
        .grade
}

fn max_grade(array: &Vec<StudentInfo>) -> i64 {
    array
        .iter()
        .max_by(|lhs, rhs| lhs.grade.cmp(&rhs.grade))
        .unwrap()
        .grade
}

fn average_grade(array: &Vec<StudentInfo>) -> f64 {
    array.iter().fold(0, |sum, student| sum + student.grade) as f64 / array.len() as f64
}

fn main() {
    println!("Hello, world!");
}
