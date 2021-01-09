use std::fs::File;
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

fn check_words_average_len(context: &str) -> f64 {
    let mut average = 0.;
    let mut amount = 0;

    for s in context
        .split(|c: char| c.is_whitespace() || c == ',' || c == '.')
        .filter(|s| !s.is_empty())
    {
        if amount != 0 {
            average = (average * amount as f64 + s.len() as f64) / (amount + 1) as f64;
        } else {
            average += s.len() as f64;
        }
        amount += 1;
    }

    return average;
}

fn main() {
    println!(
        "Words' average length: {:.2}",
        check_words_average_len(&read_file("input.txt"))
    );
}
