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

    println!("Athlete cheat: ")
    for athlete in &athletes {
        let mut flag = false;
        for i in 0..(athlete.records.len() - 1) {
            if athlete.records[i + 1].id - athlete.records[i].id > 1 {
                if !flag {
                    print!("{}: ", athlete.id);
                    flag = true;
                }
                print!("{}->{}, ", athlete.records[i].id, athlete.records[i + 1].id);
            }
            let time = athlete.records[i + 1].time - athlete.records[i].time;
            let length = points[(athlete.records[i + 1].id) as usize]
                - points[(athlete.records[i].id) as usize];
            let speed = time.num_minutes() as f64 / length;
            if speed < 4.5 {
                if !flag {
                    print!("{}: ", athlete.id);
                    flag = true;
                }
                print!("{}->{} {:.2} mins/mile, ", athlete.records[i].id, athlete.records[i + 1].id, speed);
            }
        }
        println!("");
    }
}
