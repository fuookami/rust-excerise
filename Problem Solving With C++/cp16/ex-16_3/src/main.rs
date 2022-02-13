use scan_fmt::scan_fmt;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::string::String;

struct NotDigitError {
    string: String,
}

impl fmt::Debug for NotDigitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" is not digits.", self.string)
    }
}

impl fmt::Display for NotDigitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" is not digits.", self.string)
    }
}

impl Error for NotDigitError {}

fn parse_numer(s: &str) -> Result<u64, NotDigitError> {
    match scan_fmt!(s, "{}", u64) {
        Result::Ok(number) => {
            if number > 0 && number <= 10 {
                Result::Ok(number)
            } else {
                Result::Err(NotDigitError {
                    string: String::from_str(s).unwrap(),
                })
            }
        }
        Result::Err(_) => Result::Err(NotDigitError {
            string: String::from_str(s).unwrap(),
        }),
    }
}

fn main() {
    println!("Hello, world!");
}
