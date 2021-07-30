#[macro_use]
extern crate scan_fmt;

use chrono::NaiveTime;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Record {
    id: u64,
    time: NaiveTime,
}

struct Athlete {
    id: u64,
    name: String,
    order: Option<u32>,
    records: Vec<Record>,
}

impl Athlete {
    fn new(id: u64, name: String) -> Athlete {
        Athlete {
            id: id,
            name: name,
            order: Option::None,
            records: Vec::new(),
        }
    }
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

fn parse_athletes(context: &str) -> Vec<Athlete> {
    let mut athletes = Vec::new();
    for line in context.trim().split("\n").filter(|s| !s.is_empty()) {
        match scan_fmt!(line, "{},{} {}", u64, String, String) {
            Result::Ok((id, name, family_name)) => {
                athletes.push(Athlete::new(id, format!("{} {}", name, family_name)));
            }
            Result::Err(_) => panic!("Invalid format!"),
        }
    }
    athletes
}

fn parse_records(context: &str, athletes: &mut Vec<Athlete>) -> Vec<f64> {
    let mut flag = 0;
    let mut points = Vec::new();
    for line in context.trim().split("\n").filter(|s| !s.is_empty()) {
        match flag {
            0 => {
                for part in line.trim().split(" ").filter(|s| !s.is_empty()) {
                    points.push(part.parse().unwrap());
                }
            }
            1 => match scan_fmt!(line, "{} {} {}", u32, u32, u32) {
                Result::Ok((h, m, s)) => {
                    let _ = NaiveTime::from_hms(h, m, s);
                }
                Result::Err(_) => panic!("Invalid format!"),
            },
            _ => match scan_fmt!(line, "{},{},{} {} {}", u64, u64, u32, u32, u32) {
                Result::Ok((point_index, athlete_id, h, m, s)) => {
                    for athlete in &mut *athletes {
                        if athlete.id == athlete_id {
                            athlete.records.push(Record {
                                id: point_index,
                                time: NaiveTime::from_hms(h, m, s),
                            });
                        }
                    }
                }
                Result::Err(_) => panic!("Invalid format!"),
            },
        }
        flag += 1;
    }
    points
}

fn main() {
    let mut athletes = parse_athletes(&read_file("athlete.txt"));
    let points = parse_records(&read_file("data.txt"), &mut athletes);
    athletes.sort_by(|lhs: &Athlete, rhs: &Athlete| {
        lhs.records
            .last()
            .unwrap()
            .time
            .cmp(&rhs.records.last().unwrap().time)
    });
    for i in 0..athletes.len() {
        athletes[i].order = Option::Some(i as u32);
    }

    print!("Athlete rank: ");
    for athlete in &athletes {
        print!("{}, ", athlete.name);
    }
    println!("");

    println!("Athlete subsection speed: ");
    for athlete in &athletes {
        print!("{}: ", athlete.name);
        for i in 0..(athlete.records.len() - 1) {
            let time = athlete.records[i + 1].time - athlete.records[i].time;
            let length = points[(athlete.records[i + 1].id) as usize]
                - points[(athlete.records[i].id) as usize];
            print!("{:.2} mins/mile, ", time.num_minutes() as f64 / length);
        }
        println!("");
    }

    println!("Athlete speed: ");
    for athlete in &athletes {
        if athlete.records.first().unwrap().id == 0
            && athlete.records.last().unwrap().id as usize == points.len() - 1
        {
            print!("{}: ", athlete.name);
            let time = athlete.records.last().unwrap().time - athlete.records.first().unwrap().time;
            print!("{:.2} mins, ", time.num_minutes());
            let length = points.last().unwrap() - points.first().unwrap();
            print!("{:.2} mins/mile, ", time.num_minutes() as f64 / length);
            println!("");
        }
    }
}
