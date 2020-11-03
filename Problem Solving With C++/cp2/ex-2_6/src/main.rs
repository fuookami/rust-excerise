use std::io;

const HOURLY_PAY: f64 = 16.78;
const NORMAL_MAN_HOUR: i64 = 40;
const OVERTIME_MAN_HOUR_PAY_SCALE: f64 = 1.5;
const SOCIAL_INSURANCE_PERCENT: f64 = 0.06;
const FEDERAL_TAX_PERCENT: f64 = 0.14;
const STATE_TAX_PERCENT: f64 = 0.05;
const WEEKLY_UNION_DUES: f64 = 10.;
const EXTRA_MEDICIAL_INSURANCE_FAMILY_MEMBER_AMOUNT: i64 = 3;
const EXTRA_MEDICIAL_INSURANCE: f64 = 35.;

fn read_man_hour() -> i64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let man_hour: i64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match man_hour {
        man_hour if man_hour >= 0 => man_hour,
        _ => panic!("Not negative man_hour!"),
    }
}

fn read_amount() -> i64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let amount: i64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match amount {
        amount if amount >= 0 => amount,
        _ => panic!("Not negative amount!"),
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
        println!("Please input employee man hour: ");
        let man_hour = read_man_hour();

        println!("Please input employee family member amount: ");
        let amount = read_amount();

        let gross_pay = match man_hour {
            man_hour if man_hour > NORMAL_MAN_HOUR => {
                ((man_hour - NORMAL_MAN_HOUR) as f64 * OVERTIME_MAN_HOUR_PAY_SCALE
                    + NORMAL_MAN_HOUR as f64)
                    * HOURLY_PAY
            }
            man_hour => man_hour as f64 * HOURLY_PAY,
        };
        let social_insurance = gross_pay * SOCIAL_INSURANCE_PERCENT;
        let federal_tax = gross_pay * FEDERAL_TAX_PERCENT;
        let state_tax = gross_pay * STATE_TAX_PERCENT;
        let extra_medicial_insurance = match amount {
            _ if amount >= EXTRA_MEDICIAL_INSURANCE_FAMILY_MEMBER_AMOUNT => {
                EXTRA_MEDICIAL_INSURANCE
            }
            _ => 0.,
        };
        let actual_pay = gross_pay
            - social_insurance
            - federal_tax
            - state_tax
            - WEEKLY_UNION_DUES
            - extra_medicial_insurance;

        println!("Gross pay: {:.2}", gross_pay);
        println!("Social insurance: {:.2}", social_insurance);
        println!("Federal tax: {:.2}", federal_tax);
        println!("State tax: {:.2}", state_tax);
        println!("Union dues: {:.2}", WEEKLY_UNION_DUES);
        match amount {
            _ if amount >= EXTRA_MEDICIAL_INSURANCE_FAMILY_MEMBER_AMOUNT => {
                println!("Extra medical insurance: {:.2}", EXTRA_MEDICIAL_INSURANCE)
            }
            _ => {}
        }
        println!("Acutal pay: {:.2}", actual_pay);

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
