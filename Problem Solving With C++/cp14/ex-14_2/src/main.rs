fn reverse(strs: &mut Vec<char>, i: usize, j: usize) {
    if i < j {
        strs.swap(i, j);
        reverse(strs, i + 1, j - 1);
    }
}

fn main() {
    println!("Hello, world!");
}
