use std::io;
use std::mem::swap;

fn read_tempature() -> i64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let temperature: i64 = match line.trim().parse() {
        Ok(temperature) => temperature,
        Err(_) => panic!("Failed to parse to number!"),
    };
    temperature
}

fn calculate_sound_velocity(temperature: f64) -> f64 {
    331.3 + 0.61 * temperature
}

fn main() {
    println!("Please input begin temperature(℃): ");
    let mut begin_temperature = read_tempature();

    println!("Please input end temperature(℃): ");
    let mut end_temperature = read_tempature();

    if begin_temperature > end_temperature {
        swap(&mut begin_temperature, &mut end_temperature);
    }

    for i in begin_temperature..=end_temperature {
        println!(
            "At {} degrees Celsius the velocity of sound is {:.2} m/s",
            i,
            calculate_sound_velocity(i as f64)
        );
    }
}
