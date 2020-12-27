use std::io;

fn to_string(value: i64) -> String {
    let mut ret = String::new();
    match value {
        0 => ret += "zero",
        1 => ret += "one",
        2 => ret += "two",
        3 => ret += "three",
        4 => ret += "four",
        5 => ret += "five",
        6 => ret += "six",
        7 => ret += "seven",
        8 => ret += "eight",
        9 => ret += "nine",
        10 => ret += "ten",
        11 => ret += "eleven",
        12 => ret += "twelve",
        13 => ret += "thirteen",
        14 => ret += "fourteen",
        15 => ret += "fifteen",
        16 => ret += "sixteen",
        17 => ret += "seventeen",
        18 => ret += "eighteen",
        19 => ret += "nineteen",
        _ => {
            match value / 10 {
                2 => ret += "twenty",
                3 => ret += "thirty",
                4 => ret += "forty",
                5 => ret += "fifty",
                6 => ret += "sixty",
                7 => ret += "seventy",
                8 => ret += "eighty",
                9 => ret += "ninety",
                _ => panic!(""),
            }
            if value % 10 != 0 {
                ret += "-";
                ret += &to_string(value % 10);
            }
        }
    }
    unsafe {
        ret.as_mut_vec()[0].make_ascii_uppercase();
    }
    return ret;
}

fn print_lyrics(mut value: i64) {
    while value >= 0 {
        let num_str = to_string(value);
        if value != 0 {
            println!("{} bottles of beer on the wall,", num_str);
            println!("{} bottles of beer!", num_str);
            println!("Take one down, Pass it around,");
        } else {
            println!("{} bottles of beer on the wall.", num_str);
        }
        value -= 1;
    }
}

fn read_i64() -> i64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<i64>() {
        Ok(num) => match num {
            num if 0 <= num && num < 100 => num,
            _ => panic!("Invalid number!"),
        },
        Err(_) => panic!("Failed to parse to number!"),
    }
}

fn main() {
    println!("Enter value: ");
    let value = read_i64();
    print_lyrics(value);
}
