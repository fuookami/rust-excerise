#![feature(assoc_char_funcs)]

use std::fmt::*;
use std::io;
use std::ops::Add;

struct BigInteger {
    _buffer: Vec<char>,
}

impl BigInteger {
    fn new() -> BigInteger {
        BigInteger {
            _buffer: Vec::new(),
        }
    }

    fn new_from(buffer: &str) -> BigInteger {
        let mut ret = BigInteger::new();
        for ch in buffer.chars().rev() {
            if ch.is_ascii_digit() {
                ret._buffer.push(ch)
            } else {
                panic!("Invalid big integer")
            }
        }
        ret
    }
}

impl Add for &BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: &BigInteger) -> Self::Output {
        let mut flag = false;
        let mut ret = BigInteger::new();
        let mut i = 0;
        while i < std::cmp::max(self._buffer.len(), rhs._buffer.len()) || flag {
            let mut temp = 0;
            if i < self._buffer.len() {
                temp = temp + self._buffer[i].to_digit(10).unwrap();
            }
            if i < rhs._buffer.len() {
                temp = temp + rhs._buffer[i].to_digit(10).unwrap();
            }
            if flag {
                temp += 1;
                flag = false;
            }
            if temp >= 10 {
                flag = true;
                temp -= 10;
            }
            ret._buffer.push(char::from_digit(temp, 10).unwrap());
            i += 1;
        }
        ret
    }
}

impl Display for BigInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for i in (0..self._buffer.len()).rev() {
            match write!(f, "{}", self._buffer[i]) {
                Result::Ok(_) => {}
                Result::Err(err) => return Result::Err(err),
            }
        }
        Result::Ok(())
    }
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line
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
        println!("Enter lhs big integer: ");
        let num1 = BigInteger::new_from(read_line().trim());

        println!("Enter rhs big integer: ");
        let num2 = BigInteger::new_from(read_line().trim());

        let num = &num1 + &num2;
        println!("{} + {} = {}", num1, num2, num);

        println!("Continue(y)?:");
        if !read_confirm() {
            break;
        }
    }
}
