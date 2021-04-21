use rand::Rng;

fn generate_nums() -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let mut values = Vec::<u64>::new();
    for _ in 0..rng.gen_range(10..20) {
        values.push(rng.gen_range(0..100));
    }
    values
}

fn print_nums(nums: &Vec<u64>) {
    for num in nums {
        print!("{} ", num);
    }
    print!("\n");
}

fn sort(nums: &mut Vec<u64>) {
    for i in 0..(nums.len() - 1) {
        let mut min_index = i;
        let mut min = nums[i];
        for j in (i + 1)..nums.len() {
            if nums[j] < min {
                min_index = j;
                min = nums[j];
            }
        }
        nums.swap(i, min_index);
    }
}

fn main() {
    let mut nums = generate_nums();
    print_nums(&nums);
    sort(&mut nums);
    print_nums(&nums);
}
