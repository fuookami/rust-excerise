use std::io;

enum Root {
    Real(f64),
    Imaginary((f64, f64)),
}

impl Root {
    fn to_string(&self) -> String {
        match self {
            Root::Real(real) => format!("{:.2}", real).to_string(),
            Root::Imaginary((real_part, imaginary_part)) => match imaginary_part {
                imaginary_part if *imaginary_part >= 0. => {
                    format!("{:.2} + {:.2}i", real_part, imaginary_part).to_string()
                }
                imaginary_part => {
                    format!("{:.2} - {:.2}i", real_part, imaginary_part.abs()).to_string()
                }
            },
        }
    }
}

fn calculate_root(a: f64, b: f64, c: f64) -> (Root, Root) {
    let discriminant = b.powi(2) - 4. * a * c;
    match discriminant {
        delta if discriminant >= 0. => {
            let temp = delta.powf(0.5);
            (
                Root::Real((-b + temp) / (2. * a)),
                Root::Real((-b - temp) / (2. * a)),
            )
        }
        delta => {
            let real_part = -b / (2. * a);
            let imaginary_part = delta.powf(0.5) / (2. * a);
            (
                Root::Imaginary((real_part, imaginary_part)),
                Root::Imaginary((real_part, -imaginary_part)),
            )
        }
    }
}

fn read_problem() -> Option<(f64, f64, f64)> {
    let mut ret = Vec::new();
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return Option::None;
    }
    for part in line.trim().split(' ') {
        ret.push(match part.parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Option::None,
        })
    }
    match ret {
        ret if ret.len() == 3 => Option::Some((ret[0], ret[1], ret[2])),
        _ => Option::None,
    }
}

fn main() {
    loop {
        println!("Enter problem (a, b, c): ");
        let problem = read_problem();
        if problem.is_none() {
            break;
        }

        let (a, b, c) = problem.unwrap();
        let (root1, root2) = calculate_root(a, b, c);
        println!("Root: {}, {}", root1.to_string(), root2.to_string());
    }
}
