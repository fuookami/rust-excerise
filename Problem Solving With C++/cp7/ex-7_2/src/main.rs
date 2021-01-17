use std::io;
use std::str::FromStr;

fn parse_base16_char(ch: char) -> u8 {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => panic!("Invalid base16 character!"),
    }
}

fn parse_base16_value(value: u8) -> char {
    match value {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        _ => panic!("Invalid base16 value!"),
    }
}

fn get_base16_zero() -> String {
    String::from_str("0000000000").unwrap()
}

fn base16_char_plus(lhs: char, rhs: char, carry: bool) -> (char, bool) {
    let value = parse_base16_char(lhs) + parse_base16_char(rhs) + if carry { 1 } else { 0 };
    if value >= 16 {
        (parse_base16_value(value - 16), true)
    } else {
        (parse_base16_value(value), false)
    }
}

fn base16_plus(lhs: String, rhs: String) -> Result<String, &'static str> {
    let mut ret = get_base16_zero();
    let mut carry = false;
    for i in (0..10).rev() {
        let (ch, this_carry) =
            base16_char_plus(lhs.as_bytes()[i] as char, rhs.as_bytes()[i] as char, carry);
        unsafe {
            ret.as_bytes_mut()[i] = ch as u8;
        }
        carry = this_carry;
    }
    if carry {
        Result::Err("Addition Overflow")
    } else {
        Result::Ok(ret)
    }
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line
}

fn parse_base16(context: &str) -> String {
    let mut ret = get_base16_zero();
    let mut i = 9;
    for ch in context.char_indices().rev() {
        unsafe {
            ret.as_bytes_mut()[i] = ch.1 as u8;
        }
        i -= 1;
    }
    ret
}

fn main() {
    println!("Enter lhs base16 value: ");
    let lhs = parse_base16(read_line().trim());

    println!("Enter rhs base16 value: ");
    let rhs = parse_base16(read_line().trim());

    let sum = base16_plus(lhs, rhs);
    match sum {
        Result::Ok(sum) => println!("Sum: {}", sum),
        Result::Err(msg) => println!("{}", msg),
    }
}
