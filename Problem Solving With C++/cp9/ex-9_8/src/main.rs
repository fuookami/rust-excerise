use rand::Rng;

fn count(points: Vec<u64>) -> Vec<(u64, usize)> {
    let mut counter = Vec::<(u64, usize)>::new();
    for point in points {
        if counter.is_empty() {
            counter.push((point, 1));
        } else {
            for i in 0..counter.len() {
                if counter[i].0 == point {
                    counter[i].1 += 1;
                    break;
                } else if counter[i].0 > point {
                    counter.insert(i, (point, 1));
                    break;
                }
            }
        }
    }
    counter
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut points = Vec::<u64>::new();
    loop {
        let point: i64 = rng.gen_range(-1..=100);
        if point == -1 {
            break;
        }
        points.push(point as u64);
    }

    let counter = count(points);
    for (point, amount) in counter {
        println!("Number of {}'s: {}", point, amount);
    }
}
