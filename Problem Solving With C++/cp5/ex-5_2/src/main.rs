use std::io;

struct Time {
    hour: u32,
    minute: u32,
}

impl Time {
    fn new(minutes: u32) -> Time {
        Time {
            hour: minutes / 60,
            minute: minutes % 60,
        }
    }

    fn minutes(&self) -> u32 {
        self.hour * 60 + self.minute
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
        println!("Enter begin time: ");
        let begin_time = match parse_time(&read_line()) {
            Result::Ok(time) => time.minutes(),
            Result::Err(msg) => {
                println!("{}", msg);
                break;
            }
        };

        println!("Enter end time: ");
        let end_time = match parse_time(&read_line()) {
            Result::Ok(time) => time.minutes(),
            Result::Err(msg) => {
                println!("{}", msg);
                break;
            }
        };

        if begin_time < end_time {
            let duration = Time::new(end_time - begin_time);
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
