use std::io;

struct Time {
    hour: u32,
    minute: u32,
    is_am: bool,
}

struct TimeDuration {
    hour: u32,
    minute: u32,
}

impl Time {
    fn normalize(&self) -> u32 {
        self.hour * 60 + self.minute + if !self.is_am { 12 * 60 } else { 0 }
    }
}

impl TimeDuration {
    fn new(minutes: u32) -> TimeDuration {
        TimeDuration {
            hour: minutes / 60,
            minute: minutes % 60,
        }
    }
}

fn parse_time(time: &str) -> Result<Time, &'static str> {
    let mut hour = 0;
    let mut minute = 0;
    let mut is_am = Option::<usize>::None {};
    let mut flag = 0;

    for s in time.split(|c| c == ':' || c == ' ') {
        match flag {
            0 => match s.trim().parse::<u32>() {
                Ok(num) if num < 12 => hour = num,
                _ => return Result::Err("Invalid hour!"),
            },
            1 => match s.trim().parse::<u32>() {
                Ok(num) if num < 60 => minute = num,
                _ => return Result::Err("Invalid minute!"),
            },
            2 => is_am = s.trim().find(|c: char| c.to_ascii_uppercase() == 'A'),
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

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn main() {
    loop {
        println!("Enter begin time: ");
        let begin_time = match parse_time(&read_line()) {
            Result::Ok(time) => time.normalize(),
            Result::Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };

        println!("Enter end time: ");
        let end_time = match parse_time(&read_line()) {
            Result::Ok(time) => time.normalize(),
            Result::Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };

        if begin_time < end_time {
            let duration = TimeDuration::new(end_time - begin_time);
            println!("Duration: {}:{}", duration.hour, duration.minute);
        } else {
            println!("Invalid duration!");
        }

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
