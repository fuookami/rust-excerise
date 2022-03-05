#![feature(slice_group_by)]

struct MovieGrade {
    movie: String,
    grade: i64,
}

fn statistic_analyze(grades: &Vec<MovieGrade>) -> Vec<(String, usize, f64)> {
    grades
        .group_by(|lhs, rhs| lhs.movie == rhs.movie)
        .map(|this_grades| {
            (
                this_grades[0].movie.clone(),
                this_grades.len(),
                this_grades
                    .iter()
                    .fold(0, |sum, this_grade| sum + this_grade.grade) as f64
                    / this_grades.len() as f64,
            )
        })
        .collect()
}

fn main() {
    println!("Hello, world!");
}
