use std::io;

enum Pattern<'a> {
    Number(&'a str),
    Else(&'a str),
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim().to_string()
}

fn judge<'a>(part: &'a str) -> Pattern<'a> {
    match part.chars().all(|ch: char| ch.is_numeric()) {
        true => Pattern::Number(part),
        false => Pattern::Else(part),
    }
}

fn split<'a>(context: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    let bytes = context.as_bytes();
    let mut is_alpha = (bytes[0] as char).is_alphanumeric();
    let mut j = 0;
    for i in 0..bytes.len() {
        if (bytes[i] as char).is_alphanumeric() != is_alpha {
            ret.push(context.get(j..i).unwrap());
            j = i;
            is_alpha = (bytes[i] as char).is_alphanumeric();
        }
    }
    ret.push(context.get(j..).unwrap());
    ret
}

fn transform<'a>(parts: &Vec<&'a str>) -> String {
    let mut ret = String::new();
    for i in 0..parts.len() {
        match judge(parts[i]) {
            Pattern::Number(word) => {
                for _ in 0..word.len() {
                    ret += "x";
                }
            }
            Pattern::Else(word) => ret += word,
        }
    }
    ret
}

fn main() {
    println!("Enter a line of context: ");
    let context = read_line();
    println!("{}", transform(&split(&context)));
}
