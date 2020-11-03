use std::io;

fn read_integers() -> Vec<i32> {
    let mut ret = Vec::new();
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    for part in line.trim().split(' ') {
        ret.push(match part.parse::<i32>() {
            Ok(num) => num,
            Err(_) => panic!("Failed to parse to number!"),
        })
    }
    match ret {
        ret if ret.len() == 10 => ret,
        _ => panic!("Number amount is not 10!"),
    }
}

fn main() {
    println!("Please input 10 intergers: ");
    let integers = read_integers();

    println!(
        "Sum of positive intergers: {}",
        integers.iter().filter(|&&x| x >= 0).sum::<i32>()
    );
    println!(
        "Sum of negative intergers: {}",
        integers.iter().filter(|&&x| x < 0).sum::<i32>()
    );
    println!("Sum of all intergers: {}", integers.iter().sum::<i32>());
}
