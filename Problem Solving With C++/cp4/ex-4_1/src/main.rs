use std::io;

fn read_f64() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<f64>() {
        Ok(num) => match num {
            num if num > 0. => num,
            _ => panic!("Unvalid price!"),
        },
        Err(_) => panic!("Failed to parse to number!"),
    }
}

fn calculate_payment(rest_payment: f64) -> f64 {
    let payment_for_price = rest_payment * 0.03;
    payment_for_price + (rest_payment - payment_for_price) * (0.08 - 0.08 * 0.35)
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
        println!("Enter price:  ");
        let price = read_f64();

        println!("Enter down payment:");
        let down_payment = read_f64();

        println!("Payment: {}", calculate_payment(price - down_payment));

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
