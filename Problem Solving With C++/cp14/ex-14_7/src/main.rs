fn count_impl(amount: u64) -> u64 {
    let this_amount = amount / 7;
    if this_amount == 0 {
        0
    } else {
        this_amount + count_impl(this_amount + amount % 7)
    }
}

fn count(amount: u64) -> u64 {
    amount + count_impl(amount)
}

fn main() {
    println!("Hello, world!");
}
