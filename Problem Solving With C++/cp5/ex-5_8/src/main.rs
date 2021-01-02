use std::io;

struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    fn leap_year(&self) -> bool {
        self.year % 400 == 0 || (self.year % 4 == 0 && self.year % 100 != 0)
    }

    fn century_value(&self) -> u32 {
        (3 - self.year / 100 % 4) * 2
    }

    fn year_value(&self) -> u32 {
        let duration_year = self.year % 100;
        duration_year + duration_year / 4
    }

    fn month_value(&self) -> u32 {
        match self.month {
            0 => {
                if self.leap_year() {
                    6
                } else {
                    0
                }
            }
            1 => {
                if self.leap_year() {
                    2
                } else {
                    3
                }
            }
            2 => 3,
            3 => 6,
            4 => 1,
            5 => 4,
            6 => 6,
            7 => 2,
            8 => 5,
            9 => 0,
            10 => 3,
            11 => 5,
            _ => panic!("Invalid month!"),
        }
    }

    fn weekday(&self) -> u32 {
        (self.day + 1 + self.month_value() + self.year_value() + self.century_value()) % 7
    }
}

fn parse_date(time: &str) -> Result<Date, &'static str> {
    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut flag = 0;

    for s in time
        .split(|c| c == ',' || c == ' ')
        .filter(|s| !s.is_empty())
    {
        match flag {
            0 => match s.trim() {
                "January" => month = 0,
                "February" => month = 1,
                "March" => month = 2,
                "April" => month = 3,
                "May" => month = 4,
                "June" => month = 5,
                "July" => month = 6,
                "August" => month = 7,
                "September" => month = 8,
                "October" => month = 9,
                "November" => month = 10,
                "December" => month = 11,
                _ => match s.trim().parse::<u32>() {
                    Ok(num) if 0 < num && num <= 12 => month = num - 1,
                    _ => return Result::Err("Invalid month!"),
                },
            },
            1 => match s.trim().parse::<u32>() {
                Ok(num) if 0 < num && num <= 31 => day = num - 1,
                _ => return Result::Err("Invalid day!"),
            },
            2 => match s.trim().parse::<u32>() {
                Ok(num) => year = num,
                _ => return Result::Err("Invalid year!"),
            },
            _ => return Result::Err("Invalid time format!"),
        }
        flag = flag + 1;
    }

    Result::Ok(Date {
        year: year,
        month: month,
        day: day,
    })
}

fn day_of_week(line: &str) -> Result<u32, &'static str> {
    let date = parse_date(line);

    match date {
        Result::Ok(date) => Result::Ok(date.weekday()),
        Result::Err(msg) => Result::Err(msg),
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
    println!("Enter date: ");
    match day_of_week(&read_line()) {
        Result::Ok(weekday) => {
            println!(
                "Weekday: {}",
                match weekday {
                    0 => "Sunday",
                    1 => "Monday",
                    2 => "Tuesday",
                    3 => "Wednesday",
                    4 => "Thursday",
                    5 => "Friday",
                    6 => "Saturday",
                    _ => panic!("Invalid weekday!"),
                }
            )
        }
        Result::Err(msg) => println!("{}", msg),
    }
}
