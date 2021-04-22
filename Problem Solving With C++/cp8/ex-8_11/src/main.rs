use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim().to_string()
}

fn decrypt(context: &str, key: u8) -> String {
    let bytes = context.as_bytes();
    let mut ret = String::new();
    for byte in bytes {
        if byte + key > 126 {
            ret.push((32 + ((byte + key) - 127)) as char);
        } else {
            ret.push((byte + key) as char);
        }
    }
    ret
}

fn main() {
    println!("Enter ciphertext: ");
    let context = read_line();
    for i in 1..=100 {
        println!("Key = {}, {}", i, decrypt(&context, i));
    }
}
