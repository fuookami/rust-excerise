use std::io;

fn calculate_sd(values: &Vec<f64>) -> f64 {
    let average = values.iter().fold(0., |acc, x| acc + x) / values.len() as f64;
    (values.iter().fold(0., |acc, x| acc + (x - average).powi(2)) / values.len() as f64).sqrt()
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line
}

fn parse_values(context: &str) -> Vec<f64> {
    let mut ret = Vec::<f64>::new();
    for s in context
        .trim()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
    {
        match s.parse::<f64>() {
            Result::Ok(value) => ret.push(value),
            Result::Err(_) => {}
        }
    }
    ret
}

fn main() {
    println!("Enter values: ");
    let values = parse_values(&read_line());
    println!("SD: {}", calculate_sd(&values));
}
