use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim().to_string()
}

fn transform(context: &str) -> String {
    let mut parts = Vec::new();
    for part in context
        .trim()
        .split(|c: char| c == ' ')
        .filter(|s| !s.is_empty())
    {
        parts.push(part);
    }

    let mut ret = String::new();
    ret += parts.last().unwrap();
    ret += ",";
    for i in 0..(parts.len() - 1) {
        ret += " ";
        ret += parts[i];
    }
    ret
}

fn main() {
    println!("Enter your name: ");
    let name = read_line();
    println!("{}", transform(&name));
}
