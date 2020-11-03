use std::io;

const GRAM_PER_VASE: f64 = 350.;
const GRAM_PER_POUND: f64 = 454.;
const SWEET_AGENT_PERCENT: f64 = 0.001;

fn read_weight() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let weight: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match weight {
        weight if weight >= 0. => weight,
        _ => panic!("Not negative weight!"),
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
        println!("Please input mass of sweetening agent that killed the mice(gram): ");
        let sweetening_agent_of_mice_mass = read_weight();

        println!("Please input mass of mice(gram): ");
        let mice_mass = read_weight();
        let sweetening_agent_mass_per_gram = sweetening_agent_of_mice_mass / mice_mass;

        println!("Please input your expectant weight after losing weight(pound): ");
        let person_mass = read_weight() * GRAM_PER_POUND;

        let sweetening_agent_of_person_mass = person_mass * sweetening_agent_mass_per_gram;
        let max_vase =
            (sweetening_agent_of_person_mass / (GRAM_PER_VASE * SWEET_AGENT_PERCENT)).ceil() as i64;
        println!("Max vase is {}", max_vase);

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
