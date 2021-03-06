#![feature(str_split_as_str)]

#[macro_use]
extern crate strum_macros;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

#[derive(Clone, Copy, AsRefStr, PartialEq, Eq)]
#[repr(u8)]
enum Weekday {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

struct Coach {
    name: String,
    scheduled_time: Vec<(Weekday, u8)>,
}

impl Coach {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            scheduled_time: Vec::new(),
        }
    }

    fn is_leisure(&self, day: Weekday, time: u8) -> bool {
        !self
            .scheduled_time
            .iter()
            .any(|x| (*x).0 == day && (*x).1 == time)
    }
}

struct Schedule {
    coachs: Vec<Coach>,
}

impl Schedule {
    fn new(names: &Vec<String>) -> Self {
        let mut coachs = Vec::new();
        for name in names {
            coachs.push(Coach::new(&name));
        }
        Self { coachs: coachs }
    }

    fn write_to_file(&self, path: &str) {
        let mut context = String::new();
        for coach in &self.coachs {
            context.push_str(&format!("{},", coach.name));
            for time in &coach.scheduled_time {
                context.push_str(&format!("{} {},", time.0 as u8, time.1));
            }
            context.push('\n');
        }
        write_file(path, &context);
    }

    fn read_from_file(path: &str) -> Self {
        let context = read_file(path);
        let mut coachs = Vec::new();
        for line in context.trim().split("\n").filter(|s| !s.is_empty()) {
            let mut it = line.split(",").filter(|s| !s.is_empty());
            let mut coach = Coach::new(it.next().unwrap());
            for part in it {
                let mut day = 0;
                let mut time = 0;
                let mut flag = 0;
                for day_time in part.split(" ") {
                    match flag {
                        0 => {
                            day = match day_time.parse::<u8>() {
                                Result::Ok(value) => value,
                                Result::Err(_) => panic!("Invalid day time!"),
                            }
                        }
                        1 => {
                            time = match day_time.parse::<u8>() {
                                Result::Ok(value) => value,
                                Result::Err(_) => panic!("Invalid day time!"),
                            }
                        }
                        _ => panic!("Invalid day time!"),
                    }
                    flag += 1
                }
                coach
                    .scheduled_time
                    .push((unsafe { std::mem::transmute::<u8, Weekday>(day) }, time));
            }
            coachs.push(coach);
        }
        Self { coachs: coachs }
    }

    fn schedule(&mut self, name: &str, day: Weekday, time: u8) -> bool {
        for coach in &mut self.coachs {
            if coach.name == name {
                if !coach.is_leisure(day, time) {
                    return false;
                }
                coach.scheduled_time.push((day, time));
                return true;
            }
        }
        false
    }

    fn leisure_coach_amount(&self, day: Weekday, time: u8) -> usize {
        self.coachs
            .iter()
            .filter(|x| x.is_leisure(day, time))
            .count()
    }

    fn individual_lesson_time(&self) -> Vec<(Weekday, u8)> {
        let mut times = Vec::new();
        for day in 1..5 {
            let weekday = unsafe { std::mem::transmute::<u8, Weekday>(day) };
            for time in 11..15 {
                if self.leisure_coach_amount(weekday, time) >= 1 {
                    times.push((weekday, time));
                }
            }
        }
        times
    }

    fn group_lesson_time(&self) -> Vec<(Weekday, u8)> {
        let mut times = Vec::new();
        for day in 1..5 {
            let weekday = unsafe { std::mem::transmute::<u8, Weekday>(day) };
            for time in 11..15 {
                if self.leisure_coach_amount(weekday, time) >= 2 {
                    times.push((weekday, time));
                }
            }
        }
        times
    }
}

fn main() {
    let coach_names = vec!["Jeff".to_string(), "Anna".to_string()];
    let mut schedule = Schedule::new(&coach_names);
    let mut rng = rand::thread_rng();

    for _ in 0..20 {
        let coach = &coach_names[rng.gen_range(0..2)];
        let day = unsafe { std::mem::transmute::<u8, Weekday>(rng.gen_range(1..5)) };
        let time = rng.gen_range(11..15);
        schedule.schedule(&coach, day, time);
    }
    schedule.write_to_file("schedule.txt");

    let schedule = Schedule::read_from_file("schedule.txt");

    print!("Individual lesson time: ");
    for time in schedule.individual_lesson_time() {
        print!("{} {},", time.0.as_ref(), time.1);
    }
    print!("\n");

    print!("Group lesson time: ");
    for time in schedule.group_lesson_time() {
        print!("{} {},", time.0.as_ref(), time.1);
    }
    print!("\n");
}
