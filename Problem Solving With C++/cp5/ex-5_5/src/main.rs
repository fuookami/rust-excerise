use std::io;

fn calculate_wci(wind_speed: f64, temperature: f64) -> f64 {
    match temperature {
        temperature if -273.15 < temperature && temperature <= 10.0 => {
            13.12 + 0.6215 * temperature - 11.37 * wind_speed.powf(0.16)
                + 0.3965 * temperature * wind_speed.powf(0.016)
        }
        _ => panic!("Invalid temperature!"),
    }
}

fn read_f64() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    }
}

fn main() {
    println!("Enter wind speed: ");
    let wind_speed = read_f64();
    if wind_speed < 0.0 {
        panic!("Invalid wind wpeed!");
    }

    println!("Enter temperature: ");
    let temperature = read_f64();

    println!("WCI: {}", calculate_wci(wind_speed, temperature));
}
