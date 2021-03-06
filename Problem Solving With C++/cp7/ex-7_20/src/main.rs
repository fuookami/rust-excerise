use rand::Rng;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

enum Gender {
    Male,
    Female,
}

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

fn read_death_probalility(context: &str) -> (HashMap<u8, f64>, HashMap<u8, f64>) {
    let mut male_death_probalility = HashMap::new();
    let mut female_death_probalility = HashMap::new();
    for line in context.split("\n").filter(|s| !s.is_empty()) {
        let mut age = 0;
        let mut m = 0.;
        let mut f = 0.;
        let mut flag = 0;
        for part in line.split_ascii_whitespace().filter(|s| !s.is_empty()) {
            match flag {
                0 => {
                    age = match part.parse::<u8>() {
                        Result::Ok(num) => num,
                        Result::Err(_) => panic!("Invalid age!"),
                    }
                }
                1 => {
                    m = match part.parse::<f64>() {
                        Result::Ok(num) => num,
                        Result::Err(_) => panic!("Invalid death probalility!"),
                    }
                }
                2 => {
                    f = match part.parse::<f64>() {
                        Result::Ok(num) => num,
                        Result::Err(_) => panic!("Invalid death probalility!"),
                    }
                }
                _ => panic!("Invalid death probalility data!"),
            }
            flag += 1;
        }
        male_death_probalility.insert(age, m);
        female_death_probalility.insert(age, f);
    }
    (male_death_probalility, female_death_probalility)
}

fn simulate(mut age: u8, death_probability: &HashMap<u8, f64>) -> u8 {
    let mut rng = rand::thread_rng();
    let mut dead = false;
    while !dead {
        match death_probability.get(&age) {
            Option::Some(pct) => {
                if rng.gen_range((0.)..(1.)) < *pct {
                    dead = true;
                } else {
                    age += 1
                }
            }
            Option::None => break,
        }
    }
    age
}

fn read_gender() -> Gender {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    if line.trim().starts_with("M") || line.trim().starts_with('m') {
        Gender::Male
    } else {
        Gender::Female
    }
}

fn read_age() -> u8 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<u8>() {
        Result::Ok(num) => num,
        Result::Err(_) => panic!("Invalid age!"),
    }
}

fn main() {
    let (male_death_probalility, female_death_probalility) =
        read_death_probalility(&read_file("LifeDeathProbability.txt"));

    println!("Enter gender: ");
    let gender = read_gender();

    println!("Enter age: ");
    let age = read_age();

    println!(
        "Alive to {} years old",
        match gender {
            Gender::Male => simulate(age, &male_death_probalility),
            Gender::Female => simulate(age, &female_death_probalility),
        }
    );
}
