use std::io;

struct Time {
    hour: u32,
    minute: u32,
    is_am: bool,
}

impl Time {
    fn normalize(&self) -> u32 {
        self.hour * 60 + self.minute + if self.is_am { 12 * 60 } else { 0 }
    }

    fn next_day_normalize(&self) -> u32 {
        self.normalize() + 24 * 60
    }
}

fn parse_time(time: &str) -> Result<Time, &'static str> {
    let mut hour = 0;
    let mut minute = 0;
    let mut is_am = Option::<usize>::None {};
    let mut flag = 0;

    for s in time.split(|c| c == ':' || c == ' ') {
        match flag {
            0 => match s.parse::<u32>() {
                Ok(num) if num < 12 => hour = num,
                _ => return Result::Err("Invalid hour!"),
            },
            1 => match s.parse::<u32>() {
                Ok(num) if num < 60 => minute = num,
                _ => return Result::Err("Invalid minute!"),
            },
            2 => is_am = s.find(|c: char| c.to_ascii_uppercase() == 'A'),
            _ => return Result::Err("Invalid time format!"),
        }
        flag = flag + 1;
    }
    match is_am {
        Option::Some(value) if value == 0 => Result::Ok(Time {
            hour: hour,
            minute: minute,
            is_am: true,
        }),
        _ => Result::Ok(Time {
            hour: hour,
            minute: minute,
            is_am: false,
        }),
    }
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    return line;
}

fn main() {
    println!("Enter today time: ");
    let today_time = parse_time(&read_line());

    println!("Enter tomorow: time: ");
    let tomorow_time = parse_time(&read_line());

    match today_time {
        Result::Ok(today_time) => match tomorow_time {
            Result::Ok(tomorow_time) => { println!("Jump {} minutes!", tomorow_time.next_day_normalize() - today_time.normalize()) }
            Result::Err(msg) => println!("{}", msg),
        },
        Result::Err(msg) => println!("{}", msg),
    }
}
