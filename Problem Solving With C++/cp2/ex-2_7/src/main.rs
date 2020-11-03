use std::io;

fn read_price() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let price: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match price {
        price if price >= 0. => price,
        _ => panic!("Not negative price!"),
    }
}

fn read_year() -> i64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let year: i64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match year {
        year if year >= 0 => year,
        _ => panic!("Not negative year!"),
    }
}

fn read_inflation_rate() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    }
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
        println!("Please input price: ");
        let mut price = read_price();

        println!("Please input years: ");
        let years = read_year();

        println!("Please input inflation rate(%): ");
        let inflation_rate = read_inflation_rate() / 100.;

        for _ in 0..years {
            price = price * (1. + inflation_rate);
        }

        println!("Price after {} years: {:.2}", years, price);

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
