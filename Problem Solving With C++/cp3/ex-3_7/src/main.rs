use std::io;

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

fn exp(x: f64) {
    let mut sum = 1.;
    let mut this_class = 1.;
    for i in 0..100 {
        this_class *= x / (i + 1) as f64;
        sum += this_class;
        print!("{:.15} ", sum);

        if i % 10 == 9 {
            print!("\n");
        }
    }
}

fn main() {
    println!("Enter x: ");
    let x = read_f64();
    exp(x);

    println!("Exp(x): {:.15}", x.exp());
}
