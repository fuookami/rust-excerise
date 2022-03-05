use std::collections::BTreeSet;

fn set_intersection<T: Ord + Clone>(lhs: &BTreeSet<T>, rhs: &BTreeSet<T>) -> BTreeSet<T> {
    lhs.iter()
        .filter(|ele| rhs.contains(ele))
        .cloned()
        .collect()
}

fn main() {
    println!("Hello, world!");
}
