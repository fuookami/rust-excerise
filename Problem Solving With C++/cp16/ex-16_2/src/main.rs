use scan_fmt::scan_fmt;
use std::error::Error;
use std::fmt;

struct MonthError {
    month: u64,
}

impl fmt::Debug for MonthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There is no such month as {}", self.month)
    }
}

impl fmt::Display for MonthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There is no such month as {}", self.month)
    }
}

impl Error for MonthError {}

struct DayError {
    month: &'static str,
    day: u64,
}

impl fmt::Debug for DayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There is no such day as {} in {}", self.day, self.month)
    }
}

impl fmt::Display for DayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There is no such day as {} in {}", self.day, self.month)
    }
}

impl Error for DayError {}

enum MonthDayParseError {
    FormatError(String),
    MonthError(MonthError),
    DayError(DayError),
}

impl fmt::Debug for MonthDayParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MonthDayParseError::MonthError(e) => e.fmt(f),
            MonthDayParseError::DayError(e) => e.fmt(f),
            MonthDayParseError::FormatError(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for MonthDayParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MonthDayParseError::MonthError(e) => e.fmt(f),
            MonthDayParseError::DayError(e) => e.fmt(f),
            MonthDayParseError::FormatError(s) => write!(f, "{}", s),
        }
    }
}

impl Error for MonthDayParseError {}

fn parse_month_day(month_day: &str) -> Result<String, MonthDayParseError> {
    const month_to_days: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const month_to_name: [&'static str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    match scan_fmt!(month_day, "{}/{}", u64, u64) {
        Result::Ok((month, day)) => {
            if (month - 1) >= 12 {
                Result::Err(MonthDayParseError::MonthError(MonthError { month: month }))
            } else if day > month_to_days[(month - 1) as usize] {
                Result::Err(MonthDayParseError::DayError(DayError {
                    month: month_to_name[(month - 1) as usize],
                    day: day,
                }))
            } else {
                Result::Ok(format!("{} {}", month_to_name[(month - 1) as usize], day))
            }
        }
        Result::Err(_) => Result::Err(MonthDayParseError::FormatError(format!(
            "No such mont_day format {}",
            month_day
        ))),
    }
}

fn main() {
    println!("Hello, world!");
}
