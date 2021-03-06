#[macro_use]
extern crate strum_macros;

use rand::Rng;

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
        !self.scheduled_time
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
