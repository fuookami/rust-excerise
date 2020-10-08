use std::io;

const G : f64 = 9.8;

fn length_of_free_falling(time: f64) -> f64 {
    G * time.powi(2) / 2.
}

fn read_time() -> f64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line!");
    let time : f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!")
    };
    match time {
        time if time >= 0. => time,
        _ => panic!("Not negative time!")
    }
}

fn main() {
    println!("Please input free falling time: ");
    let time = read_time();

    println!("Length of free falling in {}s is {}m", time, length_of_free_falling(time));
}
