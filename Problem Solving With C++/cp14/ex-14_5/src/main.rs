fn move_item(i: usize, n: char, m: char) {
    print!("{}: {} -> {}", i, n, m);
}

fn hanoi(i: usize, a: char, b: char, c: char) {
    if i == 1 {
        move_item(i, 'A', 'C');
    } else {
        hanoi(i - 1, 'A', 'C', 'B');
        move_item(i, 'A', 'C');
        hanoi(i - 1, 'B', 'A', 'C');
    }
}

fn main() {
    println!("Hello, world!");
}
