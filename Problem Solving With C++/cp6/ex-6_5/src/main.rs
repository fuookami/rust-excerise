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

fn repair(context: &mut String) {
    let mut left_arrow_replace_pos = Vec::<usize>::new();
    let mut right_arrow_replace_pos = Vec::<usize>::new();

    let mut last_tail = 0;
    while let Option::Some(next_context) = context.get(last_tail..) {
        if let Option::Some(next_tail) = next_context.find(|c| c == ';') {
            let this_line = next_context.get(..next_tail).unwrap();
            if let Option::Some(cin_pos) = this_line.find("cin") {
                let mut pos = 3;
                while let Option::Some(replace_context) = this_line.get((cin_pos + pos)..) {
                    if let Option::Some(arrow_pos) = replace_context.find("<<") {
                        left_arrow_replace_pos.push(last_tail + cin_pos + pos + arrow_pos);
                        pos = pos + arrow_pos + 3;
                    } else {
                        pos = this_line.len() + 1;
                    }
                }
            } else if let Option::Some(cout_pos) = this_line.find("cout") {
                let mut pos = 4;
                while let Option::Some(replace_context) = this_line.get((cout_pos + pos)..) {
                    if let Option::Some(arrow_pos) = replace_context.find(">>") {
                        right_arrow_replace_pos.push(last_tail + cout_pos + pos + arrow_pos);
                        pos = pos + arrow_pos + 3;
                    } else {
                        pos = this_line.len() + 1;
                    }
                }
            }
            last_tail = last_tail + next_tail + 1;
        } else {
            last_tail = context.len() + 1;
        }
    }

    for pos in left_arrow_replace_pos {
        context.replace_range(pos..(pos + 2), ">>");
    }
    for pos in right_arrow_replace_pos {
        context.replace_range(pos..(pos + 2), "<<");
    }
}

fn main() {
    let mut context = read_file("input.txt");
    repair(&mut context);
    write_file("output.txt", &context);
}
