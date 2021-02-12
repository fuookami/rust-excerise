#![feature(assoc_char_funcs)]

use std::io;
use std::ops::Add;
use std::fmt::*;

const BIG_INTEGER_LEN : usize = 3;

struct BigInteger {
    _buffer: [char; BIG_INTEGER_LEN]
}

impl BigInteger {
    fn new() -> BigInteger {
        BigInteger {
            _buffer: ['0'; BIG_INTEGER_LEN]
        }
    }

    fn new_from(buffer: &str) -> BigInteger {
        let mut ret = BigInteger::new();
        let mut chars = buffer.chars().rev();
        for i in (0..BIG_INTEGER_LEN).rev() {
            match chars.next() {
                Option::Some(ch) if ch.is_ascii_digit() => ret._buffer[i] = ch,
                Option::Some(_) => panic!("Invalid big integer"),
                Option::None => break
            }
        }
        ret
    }
}

impl Add for &BigInteger {
    type Output = (BigInteger, bool);

    fn add(self, rhs: &BigInteger) -> Self::Output {
        let mut flag = false;
        let mut ret = BigInteger::new();
        for i in (0..BIG_INTEGER_LEN).rev() {
            let mut temp = self._buffer[i].to_digit(10).unwrap() + rhs._buffer[i].to_digit(10).unwrap();
            if flag {
                temp += 1;
                flag = false;
            }
            if temp >= 10 {
                flag = true;
                temp -= 10;
            }
            ret._buffer[i] = char::from_digit(temp, 10).unwrap();
        }
        (ret, flag)
    }
}

impl Display for BigInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut i = 0;
        while i < BIG_INTEGER_LEN && self._buffer[i] == '0' {
            i += 1;
        }
        if i == BIG_INTEGER_LEN {
            write!(f, "0")
        }
        else {
            for j in i..BIG_INTEGER_LEN {
                match write!(f, "{}", self._buffer[j]) {
                    Result::Ok(_) => {},
                    Result::Err(err) => return Result::Err(err)
                }
            }
            Result::Ok(())
        }
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

        let (num, flag) = &num1 + &num2;
        println!("{} + {} = {}({})", num1, num2, num, flag);

        println!("Continue(y)?:");
        if !read_confirm() {
            break;
        }
    }
}
