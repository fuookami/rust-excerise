use scan_fmt::scan_fmt;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::string::String;

struct TimeFormatMistake {
    time: String,
}

impl fmt::Debug for TimeFormatMistake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There is no such time as {}", self.time)
    }
}

impl fmt::Display for TimeFormatMistake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There is no such time as {}", self.time)
    }
}

impl Error for TimeFormatMistake {}

fn parse_24hour_time(time: &str) -> Result<String, TimeFormatMistake> {
    match scan_fmt!(time, "{}:{}", u64, u64) {
        Result::Ok((hour, minute)) => {
            if hour >= 24 {
                Result::Err(TimeFormatMistake {
                    time: String::from_str(time).unwrap(),
                })
            } else if minute >= 60 {
                Result::Err(TimeFormatMistake {
                    time: String::from_str(time).unwrap(),
                })
            } else {
                if hour >= 12 {
                    Result::Ok(format!("{}:{} PM", hour - 12, minute))
                } else {
                    Result::Ok(format!("{}:{} AM", hour, minute))
                }
            }
        }
        Result::Err(_) => Result::Err(TimeFormatMistake {
            time: String::from_str(time).unwrap(),
        }),
    }
}

fn main() {
    println!("Hello, world!");
}
