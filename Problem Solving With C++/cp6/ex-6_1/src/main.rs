use std::error::Error;
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
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(fin) => fin,
    };

    let mut s = String::new();
    match fin.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {}
    };

    return s;
}

fn write_file(path: &str, context: &str) {
    let path = Path::new(path);
    let display = path.display();

    let mut fout = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(fout) => fout,
    };

    match fout.write(context.as_bytes()) {
        Err(why) => panic!("couldn't write {}: {}", display, why.description()),
        Ok(_) => {}
    };
}

fn main() {
    let mut s = read_file("mail.txt");

    println!("Enter replace name: ");
    let name = read_line();
    s = s.replace("#N#", name.trim());

    write_file("mail.ret.txt", &s);
}
