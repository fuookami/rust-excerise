#[macro_use]
extern crate strum_macros;

use rand::Rng;

#[derive(AsRefStr)]
enum MPAA {
    G,
    PG,
    PG13,
    R,
}

#[derive(Clone, Copy)]
#[repr(u32)]
enum Grade {
    Terrible = 1,
    Bad = 2,
    OK = 3,
    Good = 4,
    Great = 5,
}

struct Movie {
    name: String,
    mpaa: MPAA,
    grades: Vec<Grade>,
}

impl Movie {
    fn new(name: String, mpaa: MPAA) -> Self {
        Self {
            name: name,
            mpaa: mpaa,
            grades: Vec::new(),
        }
    }

    fn add_rating(&mut self, grade: Grade) {
        self.grades.push(grade)
    }

    fn get_average(&self) -> f64 {
        match self.grades.len() {
            0 => 0.,
            _ => {
                self.grades
                    .iter()
                    .map(|x| unsafe { std::mem::transmute::<Grade, u32>(*x) } as f64)
                    .sum::<f64>()
                    / self.grades.len() as f64
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut movie1 = Movie::new("m1".to_string(), MPAA::G);
    for _ in 0..5 {
        movie1.add_rating(unsafe { std::mem::transmute(rng.gen_range(1..=5)) })
    }
    println!(
        "{}, MPAA: {}, Average: {}",
        movie1.name,
        movie1.mpaa.as_ref(),
        movie1.get_average()
    );

    let mut movie2 = Movie::new("m2".to_string(), MPAA::PG);
    for _ in 0..5 {
        movie2.add_rating(unsafe { std::mem::transmute(rng.gen_range(1..=5)) })
    }
    println!(
        "{}, MPAA: {}, Average: {}",
        movie2.name,
        movie2.mpaa.as_ref(),
        movie2.get_average()
    );
}
