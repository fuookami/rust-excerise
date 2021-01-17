use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut fin = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(fin) => fin,
    };

    let mut s = String::new();
    match fin.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    };

    return s;
}

fn check_words_amount(context: &str) -> usize {
    context
        .split(|c: char| {
            c.is_whitespace() || c == ',' || c == '.' || c == ';' || c == ':' || c == '?'
        })
        .filter(|s| !s.is_empty())
        .count()
}

fn read_file_name() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line
}

fn main() {
    println!("Enter file name: ");
    let file_name = read_file_name();
    println!(
        "Words amount: {}",
        check_words_amount(&read_file(file_name.trim()))
    );
}
