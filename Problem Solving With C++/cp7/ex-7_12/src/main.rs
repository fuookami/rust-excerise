use std::io;

fn read() -> Vec<i64> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let mut nums = Vec::new();
    for s in line.trim().split_whitespace() {
        match s.parse::<i64>() {
            Result::Ok(num) => nums.push(num),
            Result::Err(_) => break,
        }
    }
    nums
}

fn total(amounts: &Vec<i64>) -> i64 {
    amounts.iter().sum()
}

fn scale(amounts: &mut Vec<i64>) {
    for i in amounts {
        *i /= 1000;
    }
}

fn main() {
    let mut amounts = Vec::new();
    println!("This program displays a graph showing production for each plant in the company.");
    loop {
        println!("Enter number of units produced by each department: ");
        let this_amount = read();
        if this_amount.is_empty() {
            break;
        }
        let total = total(&this_amount);
        println!("Total: {}", total);
        amounts.push(total);
    }
    scale(&mut amounts);

    for i in 0..amounts.len() {
        print!("#{}\t", i);
    }
    print!("\n");

    for i in 0..*amounts.iter().max().unwrap() {
        for j in &amounts {
            if i < *j {
                print!("*\t");
            } else {
                print!("\t");
            }
        }
        print!("\n");
    }
}
