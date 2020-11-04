use std::io;

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

fn read_score() -> i64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let score: i64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match score {
        score if score >= 0 => score,
        _ => panic!("Not negative score!"),
    }
}

fn main() {
    println!("How many exercises to input? ");
    let amount = read_amount();
    println!("");

    let mut total_score: i64 = 0;
    let mut total_max_score: i64 = 0;
    for i in 0..amount {
        println!("Score received for exercise {}: ", i);
        let score = read_score();

        println!("Total points possible for exercise {}: ", i);
        let max_score = read_score();

        if score > max_score {
            panic!(
                "Score greater than total points possible for exercise {}",
                i
            );
        }
        total_score += score;
        total_max_score += max_score;
        println!("");
    }

    println!(
        "Your total is {} out of {}, or {:.2}%.",
        total_score,
        total_max_score,
        (total_score as f64 / total_max_score as f64) * 100.
    );
}
