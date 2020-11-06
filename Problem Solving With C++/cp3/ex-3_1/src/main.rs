use std::io;
use std::result::Result;
use std::str::FromStr;

enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    fn workday(&self) -> bool {
        match self {
            Weekday::Saturday | Weekday::Sunday => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
struct WeekdayParseError;

impl FromStr for Weekday {
    type Err = WeekdayParseError;

    fn from_str(word: &str) -> Result<Weekday, WeekdayParseError> {
        match word {
            _ if word == "mo" => Result::Ok(Weekday::Monday),
            _ if word == "tu" => Result::Ok(Weekday::Tuesday),
            _ if word == "wd" => Result::Ok(Weekday::Wednesday),
            _ if word == "th" => Result::Ok(Weekday::Thursday),
            _ if word == "fr" => Result::Ok(Weekday::Friday),
            _ if word == "sa" => Result::Ok(Weekday::Saturday),
            _ if word == "su" => Result::Ok(Weekday::Sunday),
            _ => Result::Err(WeekdayParseError {}),
        }
    }
}

struct Record {
    hour: i64,
    minute: i64,
    last: i64,
    weekday: Weekday,
}

#[derive(Debug)]
struct RecordParseError;

impl Record {
    fn payment(&self) -> f64 {
        self.last as f64
            * match self {
                record if record.weekday.workday() => match record.hour {
                    _ if record.hour >= 8 && record.hour < 18 => 0.40,
                    _ => 0.25,
                },
                _ => 0.15,
            }
    }
}

#[derive(Debug)]
struct HourMinuteParseError;

fn read_hour_minute(s: &str) -> Result<(i64, i64), HourMinuteParseError> {
    let mut parts = s.split(':');

    let hour_part = parts.next();
    if hour_part.is_none() {
        return Result::Err(HourMinuteParseError {});
    }
    let hour_result = hour_part.unwrap().parse::<i64>();
    if hour_result.is_err() {
        return Result::Err(HourMinuteParseError {});
    }
    let hour = hour_result.unwrap();

    let minute_part = parts.next();
    if minute_part.is_none() {
        return Result::Err(HourMinuteParseError {});
    }
    let minute_result = minute_part.unwrap().parse::<i64>();
    if minute_result.is_err() {
        return Result::Err(HourMinuteParseError {});
    }
    let minute = minute_result.unwrap();

    return Result::Ok((hour, minute));
}

impl FromStr for Record {
    type Err = RecordParseError;

    fn from_str(s: &str) -> Result<Record, RecordParseError> {
        let mut parts = s.trim().split(' ');

        let weekday_part = parts.next();
        if weekday_part.is_none() {
            return Result::Err(RecordParseError {});
        }
        let weekday_result = weekday_part.unwrap().to_lowercase().parse::<Weekday>();
        if weekday_result.is_err() {
            return Result::Err(RecordParseError {});
        }
        let weekday = weekday_result.unwrap();

        let hour_minute_part = parts.next();
        if hour_minute_part.is_none() {
            return Result::Err(RecordParseError {});
        }
        let hour_minute_result = read_hour_minute(hour_minute_part.unwrap());
        if hour_minute_result.is_err() {
            return Result::Err(RecordParseError {});
        }
        let (hour, minute) = hour_minute_result.unwrap();

        let last_part = parts.next();
        if last_part.is_none() {
            return Result::Err(RecordParseError {});
        }
        let last_result = last_part.unwrap().parse::<i64>();
        if last_result.is_err() {
            return Result::Err(RecordParseError {});
        }
        let last = last_result.unwrap();

        return Result::Ok(Record {
            hour: hour,
            minute: minute,
            last: last,
            weekday: weekday,
        });
    }
}

fn read_record() -> Option<Record> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse() {
        Ok(record) => Option::Some(record),
        Err(_) => Option::None,
    }
}

fn main() {
    let mut payment: f64 = 0.;
    loop {
        println!("Enter a record: ");
        let record = read_record();

        match record {
            Option::Some(record) => payment += record.payment(),
            Option::None => break,
        };
    }

    println!("Payment: {:.2}", payment);
}
