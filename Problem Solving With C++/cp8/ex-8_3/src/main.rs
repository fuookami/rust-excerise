use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim().to_string()
}

fn get_u64(context: &str) -> u64 {
    let mut ret = 0;
    for ch in context.trim().chars() {
        ret *= 10;
        ret += match ch {
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
            _ => panic!("Invalid digit!"),
        };
    }
    ret
}

fn get_f64(context: &str) -> f64 {
    let mut integer_part = 0.;
    let mut decimal_part = 0.;
    let mut index_part = 0;
    let mut flag = 0;
    for part in context
        .trim()
        .split(|c: char| c == '.' || c == 'e')
        .filter(|s| !s.is_empty())
    {
        match flag {
            0 => integer_part = get_u64(part) as f64,
            1 => {
                decimal_part = get_u64(part) as f64;
                for _ in 0..part.len() {
                    decimal_part /= 10.;
                }
            }
            2 => index_part = get_u64(part),
            _ => panic!("Invalid floating point!"),
        }
        flag += 1;
    }

    (integer_part + decimal_part) * (10. as f64).powf(index_part as f64)
}

fn main() {
    println!("Enter floating point: ");
    let line = read_line();
    println!("{}", get_f64(&line));
}
