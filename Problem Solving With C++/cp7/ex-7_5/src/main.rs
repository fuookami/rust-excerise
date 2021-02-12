use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;

enum ReadMode
{
    FromFile,
    FromConsole
}

fn count(nums: Vec<i64>) -> BTreeMap<i64, usize> {
    let mut counter = BTreeMap::<i64, usize>::new();
    for num in nums {
        match counter.get_mut(&num) {
            Option::Some(time) => *time += 1,
            Option::None => { counter.insert(num, 1); }
        }
    }
    counter
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line
}

fn parse_values(context: &str) -> Vec<i64> {
    let mut ret = Vec::<i64>::new();
    for s in context.trim().split_ascii_whitespace().filter(|s| !s.is_empty()) {
        match s.parse::<i64>() {
            Result::Ok(value) => ret.push(value),
            Result::Err(_) => {}
        }
    }
    ret
}

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

    s
}

fn read_from_console() -> Vec<i64> {
    parse_values(&read_line())
}

fn read_from_file(path: &str) -> Vec<i64> {
    parse_values(&read_file(path))
}

fn main() {
    println!("Enter a file name or not read from console: ");
    let input = read_line();
    let mode = if input.trim().len() == 0 { ReadMode::FromConsole } else { ReadMode::FromFile };

    let counter = count(match mode {
        ReadMode::FromConsole => read_from_console(),
        ReadMode::FromFile => read_from_file(&input)
    });
    for pair in counter {
        println!("{}: {}", pair.0, pair.1)
    }
}
