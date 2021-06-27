use std::collections::HashSet;
use std::convert::From;
use std::io;

fn delete_repeats(s: &Vec<u8>) -> Vec<u8> {
    let mut cache = HashSet::<u8>::new();
    let mut i = 0;
    let mut ret = Vec::new();
    while i != s.len() {
        if !cache.contains(&s[i]) {
            cache.insert(s[i]);
            ret.push(s[i]);
        }
        i += 1;
    }
    ret
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
    let ret = delete_repeats(&Vec::<u8>::from(read_line().trim().as_bytes()));
    println!(
        "Context after deleting repeats: {}",
        String::from_utf8(ret).unwrap()
    );
}
