use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
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

fn count(s: &str) -> Vec<(char, usize)> {
    let mut counter = BTreeMap::<char, usize>::new();
    for ch in s.chars() {
        if ch == '.' {
            break;
        } else if ch.is_ascii_alphabetic() {
            match counter.get_mut(&ch) {
                Option::Some(time) => *time += 1,
                Option::None => {
                    counter.insert(ch, 1);
                }
            }
        }
    }
    Vec::from_iter(counter)
}

fn insert_sort_impl(arr: &mut Vec<(char, usize)>, len: usize) {
    let mut i = 0;
    while i < len {
        if arr[i].1 < arr[len].1 {
            break;
        }
        i += 1;
    }
    let temp = arr[len];
    for j in ((i + 1)..(len + 1)).rev() {
        arr[j] = arr[j - 1];
    }
    arr[i] = temp;
}

fn insert_sort(arr: &mut Vec<(char, usize)>) {
    for i in 1..arr.len() {
        insert_sort_impl(arr, i);
    }
}

fn print(arr: &Vec<(char, usize)>) {
    for pair in arr {
        println!("{}\t{}", pair.0, pair.1);
    }
}

fn main() {
    let context = read_file("input.txt");
    let mut counter = count(context.trim());
    insert_sort(&mut counter);
    print(&counter);
}
