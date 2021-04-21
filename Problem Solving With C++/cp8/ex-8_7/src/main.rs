use std::io;

enum Pattern<'a> {
    SheOrHe,
    HerOrHis,
    HerOrHim,
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
    if part == "She" || part == "she" || part == "He" || part == "he" {
        Pattern::SheOrHe
    } else if part == "Hers" || part == "hers" || part == "His" || part == "his" {
        Pattern::HerOrHis
    } else if part == "Her" || part == "her" || part == "Him" || part == "him" {
        Pattern::HerOrHim
    } else {
        Pattern::Else(part)
    }
}

fn split<'a>(context: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    let bytes = context.as_bytes();
    let mut is_alpha = (bytes[0] as char).is_alphabetic();
    let mut j = 0;
    for i in 0..bytes.len() {
        if (bytes[i] as char).is_alphabetic() != is_alpha {
            ret.push(context.get(j..i).unwrap());
            j = i;
            is_alpha = (bytes[i] as char).is_alphabetic();
        }
    }
    ret.push(context.get(j..).unwrap());
    ret
}

fn transform<'a>(parts: &Vec<&'a str>) -> String {
    let mut ret = String::new();
    for i in 0..parts.len() {
        match judge(parts[i]) {
            Pattern::SheOrHe => match i {
                0 => ret += "She or he",
                _ => ret += "she or he",
            },
            Pattern::HerOrHis => match i {
                0 => ret += "Her(s)",
                _ => ret += "her(s)",
            },
            Pattern::HerOrHim => match i {
                0 => ret += "Her or him",
                _ => ret += "her or him",
            },
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
