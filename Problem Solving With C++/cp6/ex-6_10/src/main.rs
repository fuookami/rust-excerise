use std::collections::HashMap;
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

    return s;
}

fn read_ranks(context: &str) -> (HashMap<String, u64>, HashMap<String, u64>) {
    let mut boys_rank = HashMap::<String, u64>::new();
    let mut girls_rank = HashMap::<String, u64>::new();
    for line in context
        .split(|c| c == '\n' || c == '\r')
        .filter(|s| !s.is_empty())
    {
        let mut rank = Option::<u64>::None;
        let mut boy_name = Option::<String>::None;
        let mut girl_name = Option::<String>::None;
        let mut flag = 0;
        for s in line
            .split(|c: char| c.is_whitespace())
            .filter(|s| !s.is_empty())
        {
            match flag {
                0 => {
                    rank = Option::Some(match s.parse::<u64>() {
                        Result::Ok(num) => num,
                        Result::Err(_) => break,
                    })
                }
                1 => {
                    boy_name = Option::Some(match String::from_str(s) {
                        Result::Ok(name) => name,
                        Result::Err(_) => break,
                    })
                }
                2 => {
                    girl_name = Option::Some(match String::from_str(s) {
                        Result::Ok(name) => name,
                        Result::Err(_) => break,
                    })
                }
                _ => break,
            }
            flag += 1;
        }
        if boy_name.is_some() && girl_name.is_some() && rank.is_some() {
            boys_rank.insert(boy_name.unwrap(), rank.unwrap());
            girls_rank.insert(girl_name.unwrap(), rank.unwrap());
        }
    }
    (boys_rank, girls_rank)
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
    let (boys_rank, girls_rank) = read_ranks(&read_file("babynames2012.txt"));
    loop {
        println!("Enter name: ");
        let name = read_line();
        match boys_rank.get(name.trim()) {
            Some(rank) => println!(
                "{} is ranked {} in popularity among boys.",
                name.trim(),
                rank
            ),
            None => println!(
                "{} is not ranked among the top 1000 boy names.",
                name.trim()
            ),
        }
        match girls_rank.get(name.trim()) {
            Some(rank) => println!(
                "{} is ranked {} in popularity among girls.",
                name.trim(),
                rank
            ),
            None => println!(
                "{} is not ranked among the top 1000 girl names.",
                name.trim()
            ),
        }

        println!("Continue?");
        if !read_confirm() {
            break;
        }
    }
}
