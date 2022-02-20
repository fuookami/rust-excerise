use std::cmp;

fn sort_impl<T: cmp::Ord>(vals: &mut Vec<T>, i: usize) {
    if i == vals.len() {
        return;
    } else {
        let mut min_index = i;
        for j in (i + 1)..vals.len() {
            if vals[j] < vals[min_index] {
                min_index = j;
            }
        }
        vals.swap(i, min_index);
        sort_impl(vals, i + 1);
    }
}

fn sort<T: cmp::Ord>(vals: &mut Vec<T>) {
    sort_impl(vals, 0);
}

fn main() {
    println!("Hello, world!");
}
