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

fn count(context: &str) -> String {
    let mut ret = String::new();
    {
        let this_result = format!("Total character amount: {}\n", context.len());
        print!("{}", this_result);
        ret.push_str(&this_result);
    }
    {
        let this_result = format!(
            "Total not empty character amount: {}\n",
            context
                .char_indices()
                .filter(|c| !c.1.is_whitespace())
                .count()
        );
        print!("{}", this_result);
        ret.push_str(&this_result);
    }
    {
        let this_result = format!(
            "Total alphabetic amount: {}\n",
            context
                .char_indices()
                .filter(|c| c.1.is_ascii_alphabetic())
                .count()
        );
        print!("{}", this_result);
        ret.push_str(&this_result);
    }
    ret
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

fn main() {
    let context = read_file("input.txt");
    write_file("output.txt", &count(&context));
}
