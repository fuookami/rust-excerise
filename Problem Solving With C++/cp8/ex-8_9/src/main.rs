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
    for i in 1..nums.len() {
        let mut j = 0;
        while j < i {
            if nums[j] > nums[i] {
                break;
            }
            j += 1;
        }
        let temp = nums[i];
        for k in ((j + 1)..(i + 1)).rev() {
            nums[k] = nums[k - 1];
        }
        nums[j] = temp;
    }
}

fn main() {
    let mut nums = generate_nums();
    print_nums(&nums);
    sort(&mut nums);
    print_nums(&nums);
}
