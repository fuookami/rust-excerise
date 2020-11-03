use std::io;

const MONTH_PER_YEAR: i64 = 12;

fn calculate_loan(target: f64, interest_rate: f64, repayment_month: i64) -> f64 {
    // x - x * rate * month / 12 = target
    // x = target / (1 - rate * month / 12)
    target / (1. - interest_rate * repayment_month as f64 / MONTH_PER_YEAR as f64)
}

fn read_target_loan() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let loan: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match loan {
        loan if loan >= 0. => loan,
        _ => panic!("Not negative target loan!"),
    }
}

fn read_interest_rate() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let interest_rate: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match interest_rate {
        interest_rate if interest_rate >= 0. => interest_rate,
        _ => panic!("Not negative interest rate!"),
    }
}

fn read_repayment_month() -> i64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let month: i64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match month {
        month if month >= 0 => month,
        _ => panic!("Not negative repayment month!"),
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
        println!("Please intpu your target loan: ");
        let target_loan = read_target_loan();

        println!("Please input interest rate(%): ");
        let interest_rate = read_interest_rate() / 100.;

        println!("Please input your repayment month: ");
        let repayment_month = read_repayment_month();

        let loan = calculate_loan(target_loan, interest_rate, repayment_month);
        let monthly_payment = loan / repayment_month as f64;

        println!("Loan: {:.2}", loan);
        println!("Monthly payment: {:.2}", monthly_payment);

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
