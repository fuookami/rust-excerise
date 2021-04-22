use chrono::NaiveTime;
use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim().to_string()
}

fn parse(context: &str) -> String {
    let time = NaiveTime::parse_from_str(context, "%I:%M %p").unwrap();
    time.format("%H%M").to_string()
}

fn main() {
    println!("Enter time: ");
    let time = parse(&read_line());
    println!("{} hours", time);
}
