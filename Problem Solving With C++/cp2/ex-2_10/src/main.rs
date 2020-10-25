use std::io;

fn read_integers() -> Vec<i32> {
    let mut ret = Vec::new();
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line!");
    for part in line.trim().split(' ') {
        ret.push(match part.parse::<i32>(){
            Ok(num) => num,
            Err(_) => panic!("Failed to parse to number!")
        })
    }
    match ret {
        ret if ret.len() == 10 => ret,
        _ => panic!("Number amount is not 10!")
    }
}

fn main() {
    println!("Please input 10 intergers: ");
    let integers = read_integers();
    
    let positive_integers = integers.iter().filter(|&x| *x >= 0).cloned().collect::<Vec<i32>>();
    let negative_integers = integers.iter().filter(|&x| *x < 0).cloned().collect::<Vec<i32>>();
    let positive_integers_sum = positive_integers.iter().sum::<i32>();
    let negative_integers_sum = negative_integers.iter().sum::<i32>();
    println!("Sum of positive intergers: {}", positive_integers_sum);
    println!("Mean of positivie integers: {}", positive_integers_sum / positive_integers.len() as i32);
    println!("Sum of negative intergers: {}", negative_integers_sum);
    println!("Mean of negative integers: {}", negative_integers_sum / negative_integers.len() as i32);
    println!("Sum of all integers: {}", integers.iter().sum::<i32>());
    println!("Mean of all integers: {}", integers.iter().sum::<i32>() / integers.len() as i32);
}
