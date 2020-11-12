use std::io;

fn read_loan() -> f64 {
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
        _ => panic!("Not negative loan!"),
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
        interest_rate if interest_rate >= 0. && interest_rate <= 100. => interest_rate / 100.,
        _ => panic!("Illegal interest rate!"),
    }
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn calculate_total_interest(mut loan: f64, interest_rate: f64) -> (i32, f64) {
    let monthly_repayment = loan / 20.;
    let mut total_interest = 0.;

    let mut month = 0;
    while loan > 0. {
        let this_month_interest = (loan * interest_rate) / 12.;
        total_interest += this_month_interest;
        let this_month_repayment = if (monthly_repayment - this_month_interest) > loan {
            loan + this_month_interest
        } else {
            monthly_repayment
        };
        loan -= this_month_repayment - this_month_interest;
        println!(
            "Month {} repayment: {:.2}, rest loan: {:.2}, ",
            month + 1,
            this_month_repayment,
            loan
        );
        month += 1;
    }
    return (month, total_interest);
}

fn main() {
    loop {
        println!("Enter loan: ");
        let loan = read_loan();

        println!("Enter interest rate: ");
        let interest_rate = read_interest_rate();

        let (month, total_interest) = calculate_total_interest(loan, interest_rate);
        let total_interest_rate = total_interest / loan;
        let total_yearly_interest_rate = total_interest_rate * 12. / month as f64;
        println!(
            "Total yearly interest rate: {:.2}%\n",
            total_yearly_interest_rate * 100.
        );

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
