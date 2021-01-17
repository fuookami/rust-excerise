use std::collections::HashSet;
use std::convert::From;
use std::io;

fn delete_repeats(mut s: Vec<u8>) -> String {
    let mut cache = HashSet::<u8>::new();
    let mut i = 0;
    while i != s.len() {
        if cache.contains(&s[i]) {
            for j in i..(s.len() - 1) {
                s[j] = s[j + 1];
                s.remove(s.len() - 1);
            }
        } else {
            cache.insert(s[i]);
        }
        i += 1;
    }
    String::from_utf8(s).unwrap()
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line
}

fn main() {
    println!("Enter context: ");
    let ret = delete_repeats(Vec::<u8>::from(read_line().trim().as_bytes()));
    println!("Context after deleting repeats: {}", ret);
}
