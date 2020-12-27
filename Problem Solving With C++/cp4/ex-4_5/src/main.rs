use std::io;

fn calculate_basal_metabolic_rate(weight: f64) -> f64 {
    70. * (weight / 2.2).powf(0.756)
}

fn calculate_activity_energy_expenditure(weight: f64, severe_degree: f64, time: f64) -> f64 {
    0.0385 * severe_degree * weight * time
}

fn calculate_required_calorie(spent_calorie: f64) -> f64 {
    spent_calorie / 0.9
}

fn calculate_required_food_amount(spent_calorie: f64, food_calorie: f64) -> i64 {
    (calculate_required_calorie(spent_calorie) / food_calorie).ceil() as i64
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
    println!("Enter weight: ");
    let weight = read_f64();

    println!("Enter activity severe degree: ");
    let severe_degree = read_f64();

    println!("Enter activity time: ");
    let time = read_f64();

    println!("Enter food calorie: ");
    let food_calorie = read_f64();

    let basal_metabolic_rate = calculate_basal_metabolic_rate(weight);
    let activity_energy_expenditure =
        calculate_activity_energy_expenditure(weight, severe_degree, time);
    let spent_calorie =
        calculate_required_calorie(basal_metabolic_rate + activity_energy_expenditure);
    println!(
        "Required amount: {}",
        calculate_required_food_amount(spent_calorie, food_calorie)
    );
}
