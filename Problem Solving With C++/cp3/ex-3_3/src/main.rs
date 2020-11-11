use std::io;

fn parse_rome_number(rome_number: &str) -> Option<i64> {
    static ROME_NUMBER_DICT: [(&str, i64); 13] = [
        ("M",   1000),
        ("CM",  900),
        ("D",   500),
        ("CD",  400),
        ("C",   100),
        ("XC",  90),
        ("L",   50),
        ("XL",  40),
        ("X",   10),
        ("IX",  9),
        ("V",   5),
        ("IV",  4),
        ("I",   1),
    ];

    if rome_number.is_empty() {
        return Option::Some(0);
    }
    for rome_number_item in &ROME_NUMBER_DICT {
        if rome_number.starts_with(rome_number_item.0) {
            let sub = rome_number.get(rome_number_item.0.len()..);
            return match sub {
                Option::Some(sub_rome_number) => {
                    let sub_number = parse_rome_number(sub_rome_number);
                    match sub_number {
                        Option::Some(number) => Option::Some(number + rome_number_item.1),
                        Option::None => Option::None,
                    }
                }
                Option::None => Option::None,
            };
        }
    }
    panic!("Bad parsing!");
}

fn read_rome_number() -> Option<i64> {
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return Option::None;
    }
    parse_rome_number(line.trim())
}

fn main() {
    loop {
        println!("Enter rome number: ");
        let value = read_rome_number();
        match value {
            Option::Some(value) => println!("Value: {}", value),
            Option::None => break,
        };
    }
}
