fn reverse(s: &mut Vec<u8>) {
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn main() {
    let mut s = "Hello world!".to_string();
    unsafe{ reverse(s.as_mut_vec()) };
    println!("{}", s);
}
