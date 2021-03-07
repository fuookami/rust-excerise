use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim().to_string()
}

fn adjust(line: &str) -> String {
    let mut ret = line
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>()
        .join(" ");
    let front_letter = (ret.as_bytes()[0] as char).to_uppercase().to_string();
    unsafe {
        ret.as_mut_vec()[0] = front_letter.as_bytes()[0];
    }
    ret
}

fn main() {
    println!("Enter a line: ");
    let line = read_line();
    println!("{}", adjust(&line));
}
