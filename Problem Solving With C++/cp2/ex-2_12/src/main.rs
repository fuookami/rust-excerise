use std::f64;
use std::io;

const GALLON_PER_CUBIC_FEET: f64 = 7.48;
const INCH_PER_FEET: f64 = 12.;

fn read_radius() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let radius: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match radius {
        radius if radius >= 0. => radius,
        _ => panic!("Not negative radius!"),
    }
}

fn read_depth() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let depth: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match depth {
        depth if depth >= 0. => depth,
        _ => panic!("Not negative depth!"),
    }
}

fn calculate_volume(radius: f64, depth: f64) -> f64 {
    f64::consts::PI * (radius / INCH_PER_FEET).powi(2) * depth * GALLON_PER_CUBIC_FEET
}

fn main() {
    println!("Enter radius(feet): ");
    let radius = read_radius();

    println!("Enter depth(inch): ");
    let depth = read_depth();

    println!("Volume: {:.0} gallon", calculate_volume(radius, depth));
}
