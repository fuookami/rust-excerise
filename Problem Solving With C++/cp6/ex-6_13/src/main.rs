use rand::Rng;
use std::fs::File;
use std::io;
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

    s
}

fn generate_guess(context: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut words = Vec::<&str>::new();
    for s in context
        .split(|c: char| c.is_whitespace())
        .filter(|s| !s.is_empty())
    {
        if s.ends_with("dous") && s != "tremendous" && s != "stupendous" && s != "horrendous" {
            words.push(s);
        }
    }
    String::from_str(words[rng.gen_range(0..words.len())]).unwrap()
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn main() {
    let guess = generate_guess(&read_file("words/words_alpha.txt"));
    println!("guess: {}", guess);
    loop {
        println!("Enter your guess: ");
        let this_guess = read_line();
        if this_guess.trim() == guess {
            println!("Success.");
            break;
        }

        println!("Fail, continue?");
        if !read_confirm() {
            break;
        }
    }
}
