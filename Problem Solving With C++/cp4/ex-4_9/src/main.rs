use rand::Rng;

fn pick_one(number: u64) -> u64 {
    match number {
        number if number < 20 => 1,
        number if 20 <= number && number < 30 => 2,
        number if 30 <= number && number < 36 => 3,
        number if 36 < number && number < 40 => 4,
        _ => panic!("Invalid pick!"),
    }
}

fn pick() -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let mut ret = Vec::<u64>::new();

    while ret.len() != 4 {
        let this_pick = pick_one(rng.gen_range(0..40));
        if !ret.contains(&this_pick) {
            ret.push(this_pick);
        }
    }
    ret
}

fn main() {
    let ret = pick();
    for i in 0..ret.len() {
        let pick = ret[i];
        match pick {
            1 => println!("Last place picks {}.", i + 1),
            2 => println!("Second-to-last picks {}.", i + 1),
            3 => println!("Thrid-to-last picks {}.", i + 1),
            4 => println!("Fourth-to-last picks {}.", i + 1),
            _ => panic!("Invalid pick!"),
        }
    }
}
