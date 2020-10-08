use std::io;

const PROMOTION_PERCENT : f64 = 0.076;
const MONTH_PER_YEAR : i64 = 12;
const SUPPLYING_AGAIN_MONTHS : i64 = 6;

fn read_annual_pay() -> f64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line!");
    let annual_pay : f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!")
    };
    match annual_pay {
        annual_pay if annual_pay >= 0. => annual_pay,
        _ => panic!("Not negative annual_pay!")
    }
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn main() {
    loop {
        println!("Please input annual pay of employee in last year: ");
        let annual_pay = read_annual_pay();
        let monthly_pay = annual_pay / MONTH_PER_YEAR as f64;

        let new_annual_pay = annual_pay * (1. + PROMOTION_PERCENT);
        let new_monthly_pay = new_annual_pay / MONTH_PER_YEAR as f64;
        let supplying_again_pay = (new_monthly_pay - monthly_pay) * SUPPLYING_AGAIN_MONTHS as f64;

        println!("Supplying anain pay: {:.2}", supplying_again_pay);
        println!("New annual pay: {:.2}", new_annual_pay);
        println!("New month pay: {:.2}", new_monthly_pay);

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue
        }
    }
}
