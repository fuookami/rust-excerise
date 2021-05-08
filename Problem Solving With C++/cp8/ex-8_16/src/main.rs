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
    records: Vec<Record>,
}

impl Athlete {
    fn new(id: u64) -> Athlete {
        Athlete {
            id: id,
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

fn parse(context: &str) -> (Vec<f64>, Vec<Athlete>) {
    let mut flag = 0;
    let mut points = Vec::new();
    let mut _time = NaiveTime::from_hms(0, 0, 0);
    let mut athletes = Vec::<Athlete>::new();
    for line in context.trim().split("\n").filter(|s| !s.is_empty()) {
        match flag {
            0 => {
                for part in line.trim().split(" ").filter(|s| !s.is_empty()) {
                    points.push(part.parse().unwrap());
                }
            }
            1 => _time = NaiveTime::parse_from_str(line.trim(), "%H %M %S").unwrap(),
            _ => {
                let mut point_index = 0;
                let mut athlete_id = 0;
                let mut time = NaiveTime::from_hms(0, 0, 0);

                let mut flag = 0;
                for part in line.trim().split(",").filter(|s| !s.is_empty()) {
                    match flag {
                        0 => point_index = part.parse().unwrap(),
                        1 => athlete_id = part.parse().unwrap(),
                        2 => time = NaiveTime::parse_from_str(part, "%H %M %S").unwrap(),
                        _ => panic!("Invalid format!"),
                    }
                    flag += 1;
                }

                let mut exist = false;
                for athlete in &mut athletes {
                    if athlete.id == athlete_id {
                        athlete.records.push(Record {
                            id: point_index,
                            time: time,
                        });
                        exist = true;
                    }
                }
                if !exist {
                    let mut athlete = Athlete::new(athlete_id);
                    athlete.records.push(Record {
                        id: point_index,
                        time: time,
                    });
                    athletes.push(athlete);
                }
            }
        }
        flag += 1;
    }
    (points, athletes)
}

fn main() {
    let (points, mut athletes) = parse(&read_file("data.txt"));
    athletes.sort_by(|lhs: &Athlete, rhs: &Athlete| {
        lhs.records
            .last()
            .unwrap()
            .time
            .cmp(&rhs.records.last().unwrap().time)
    });

    print!("Athlete rank: ");
    for athlete in &athletes {
        print!("{}, ", athlete.id);
    }
    println!("");

    println!("Athlete subsection speed: ");
    for athlete in &athletes {
        print!("{}: ", athlete.id);
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
            print!("{:.2}: ", athlete.id);
            let time = athlete.records.last().unwrap().time - athlete.records.first().unwrap().time;
            print!("{:.2} mins, ", time.num_minutes());
            let length = points.last().unwrap() - points.first().unwrap();
            print!("{:.2} mins/mile, ", time.num_minutes() as f64 / length);
            println!("");
        }
    }
}
