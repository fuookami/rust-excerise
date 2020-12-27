use std::io;

enum PaymentResult {
    Rest(f64),
    Change(f64),
}

fn pay_impl(rest: f64, payment: f64) -> PaymentResult {
    if payment >= rest {
        PaymentResult::Change(payment - rest)
    } else {
        PaymentResult::Rest(rest - payment)
    }
}

fn pay(rest: f64, payment: f64) -> Result<PaymentResult, ()> {
    match payment {
        payment if payment == 1.0 || payment == 0.25 || payment == 0.1 || payment == 0.05 => {
            Result::Ok(pay_impl(rest, payment))
        }
        _ => Result::Err(()),
    }
}

fn read_f64() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<f64>() {
        Ok(num) => match num {
            num if num > 0. => num,
            _ => panic!("Not negative!"),
        },
        Err(_) => panic!("Failed to parse to number!"),
    }
}

fn main() {
    let mut rest = 3.5;
    loop {
        println!("Enter your payment: ");
        let payment = read_f64();

        match pay(rest, payment) {
            Result::Ok(result) => match result {
                PaymentResult::Rest(now_rest) => {
                    println!("Total payment: {}", 3.5 - now_rest);
                    rest = now_rest;
                }
                PaymentResult::Change(change) => {
                    println!("Enjoy your crispy fried yogurt!");
                    println!("Change you {}.", change);
                    break;
                }
            },
            Result::Err(_) => println!("Invalid payment!"),
        };
    }
}
