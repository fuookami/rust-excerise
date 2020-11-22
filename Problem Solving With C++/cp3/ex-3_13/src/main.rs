use std::ops::Range;

fn find_locations() -> Vec<u64> {
    let mut ret = Vec::new();
    let range = Range::<usize> {
        start: 1000,
        end: 10000,
    };
    for i in range.filter(|x| x % 2 == 1) {
        let n1 = (i / 1000) as u64;
        let n2 = (i % 1000 / 100) as u64;
        let n3 = (i % 100 / 10) as u64;
        let n4 = (i % 10) as u64;
        if n1 == n2 || n1 == n3 || n1 == n4 || n2 == n3 || n2 == n4 || n3 == n4 {
            continue;
        }
        if n2 == 0 || n1 / n2 != 3 || n1 % n2 != 0 {
            continue;
        }
        if n1 + n2 + n3 + n4 != 27 {
            continue;
        }
        ret.push(i as u64);
    }
    ret
}

fn main() {
    print!("Locations: ");
    for loc in find_locations() {
        print!("{}, ", loc);
    }
    print!("\n");
}
