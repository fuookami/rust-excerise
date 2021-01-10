use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

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

fn write_file(path: &str, context: &str) {
    let path = Path::new(path);
    let display = path.display();

    let mut fout = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(fout) => fout,
    };

    match fout.write(context.as_bytes()) {
        Err(why) => panic!("couldn't write {}: {}", display, why),
        Ok(_) => {}
    };
}

fn add_line_indexes(lines: &Vec<String>) -> String {
    let mut ret = String::new();
    let total_digits = (lines.len() as f64).log(10.0).floor() as u64 + 1;
    let mut current_row: u64 = 1;
    for line in lines {
        let mut prefix = String::new();
        let this_digits = (current_row as f64).log(10.0).floor() as u64;
        for _ in 0..(total_digits - this_digits) {
            prefix.push(' ');
        }
        prefix.push_str(&format!("{}: ", current_row));
        println!("{}{}", prefix, line);
        ret.push_str(&prefix);
        ret.push_str(&line);
        ret.push('\n');
        current_row += 1;
    }
    ret
}

fn main() {
    let context = read_file("input.txt");
    let lines = context
        .split(|c| c == '\n' || c == '\r')
        .filter(|s| !s.is_empty())
        .map(|s| match String::from_str(s.trim_start()) {
            Result::Ok(s) => s,
            Result::Err(_) => panic!("Invalid context!"),
        })
        .collect::<Vec<String>>();
    write_file("output.txt", &add_line_indexes(&lines));
}
