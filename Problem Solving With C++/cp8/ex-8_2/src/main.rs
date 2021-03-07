use std::collections::BTreeMap;
use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim().to_string()
}

fn count(line: &str) -> (usize, BTreeMap<char, usize>) {
    let mut words_counter = 0;
    let mut letter_counter = BTreeMap::<char, usize>::new();
    for word in line
        .trim()
        .split(|c: char| c.is_whitespace() || c == '.' || c == ',')
        .filter(|s| !s.is_empty())
    {
        words_counter += 1;
        for ch in word.chars() {
            if ch.is_ascii_alphabetic() {
                let key = ch.to_lowercase().to_string().as_bytes()[0] as char;
                *letter_counter.entry(key).or_insert(0) += 1;
            }
        }
    }
    (words_counter, letter_counter)
}

fn main() {
    println!("Enter line: ");
    let line = read_line();
    let (words_amount, letters_amount) = count(&line);
    println!("{} words", words_amount);
    for (k, v) in letters_amount {
        println!("{} {}", v, k);
    }
}
