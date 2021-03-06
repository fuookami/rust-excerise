use rand::Rng;

use std::io;

const PIN_LEN: usize = 5;
type Pin = [u8; PIN_LEN];
const NUM_LEN: usize = 10;
type PinMap = [u8; NUM_LEN];

fn gen_map() -> PinMap {
    let mut rng = rand::thread_rng();
    let mut map: PinMap = unsafe { std::mem::zeroed() };
    for i in 0..NUM_LEN {
        map[i] = rng.gen_range(0..NUM_LEN) as u8;
    }
    map
}

fn map_pins(pins: &Pin, map: &PinMap) -> Pin {
    let mut mapped_pins: Pin = unsafe { std::mem::zeroed() };
    for i in 0..PIN_LEN {
        mapped_pins[i] = map[pins[i] as usize];
    }
    mapped_pins
}

fn read_pins() -> Pin {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let mut ret: Pin = unsafe { std::mem::zeroed() };
    let mut i = 0;
    for s in line.trim().as_bytes() {
        if s.is_ascii_digit() {
            ret[i] = *s - '0' as u8;
        } else {
            panic!("Invalid pins!");
        }
        i += 1;
    }
    ret
}

fn main() {
    let map = gen_map();
    print!("PIN: ");
    for i in 0..10 {
        print!("{} ", i);
    }
    print!("\n");
    print!("NUM: ");
    for i in 0..10 {
        print!("{} ", map[i]);
    }
    print!("\n");

    const PINS: Pin = [1, 2, 3, 4, 5];
    let mapped_pins = map_pins(&PINS, &map);
    println!("Enter pins: ");
    let pins = read_pins();
    if mapped_pins == pins {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}
