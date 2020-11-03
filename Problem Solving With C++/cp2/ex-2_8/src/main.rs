const MONTHLY_INTEREST_RATE: f64 = 0.015;
const PAYMENT_PER_MONTH: f64 = 50.;

fn main() {
    let mut loan: f64 = 1000.;
    let mut month: i64 = 0;
    let mut total_payment: f64 = 0.;

    while loan > 0. {
        let this_interest = loan * MONTHLY_INTEREST_RATE;
        if loan + this_interest < PAYMENT_PER_MONTH {
            total_payment += loan + this_interest;
            loan = 0.;
        } else {
            total_payment += PAYMENT_PER_MONTH;
            loan -= PAYMENT_PER_MONTH - this_interest;
        }
        month += 1;
    }

    println!("Month: {}", month);
    println!("Interest: {}", total_payment - loan);
}
