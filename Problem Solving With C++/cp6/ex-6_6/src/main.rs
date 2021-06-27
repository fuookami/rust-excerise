use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line
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

    return s;
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn main() {
    let mut current_index: u64 = 18;

    let f = std::io::stdout();

    loop {
        println!("Enter your question: ");
        let _ = read_line();

        println!(
            "{}",
            read_file("input.txt").replace("#N", &format!("{}", current_index))
        );

        current_index -= 1;
        if current_index == 0 {
            current_index = 18;
        }

        println!("Continue?");
        if !read_confirm() {
            break;
        }
    }
}
