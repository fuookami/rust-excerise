use std::io;

struct Time {
    hour: u32,
    minute: u32,
}

impl Time {
    fn to_12_hours_system(&self) -> String {
        if self.hour > 12 {
            format!("{:2}:{:2} PM", self.hour - 12, self.minute)
        } else {
            format!("{:2}:{:2} AM", self.hour, self.minute)
        }
    }
}

fn parse_time(time: &str) -> Result<Time, &'static str> {
    let mut hour = 0;
    let mut minute = 0;
    let mut flag = 0;

    for s in time.split(":") {
        match flag {
            0 => match s.trim().parse::<u32>() {
                Ok(num) if num < 24 => hour = num,
                _ => return Result::Err("Invalid hour!"),
            },
            1 => match s.trim().parse::<u32>() {
                Ok(num) if num < 60 => minute = num,
                _ => return Result::Err("Invalid minute!"),
            },
            _ => return Result::Err("Invalid time format!"),
        }
        flag = flag + 1;
    }

    Result::Ok(Time {
        hour: hour,
        minute: minute,
    })
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
        println!("Enter time: ");
        let time = parse_time(&read_line());

        match time {
            Result::Ok(time) => println!("12 hours system: {}", time.to_12_hours_system()),
            Result::Err(msg) => println!("{}", msg),
        }

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
