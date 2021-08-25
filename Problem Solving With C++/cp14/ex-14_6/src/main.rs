fn count_impl(costs: &Vec<u64>, i: usize) -> u64 {
    if i == costs.len() - 1 {
        costs[costs.len() - 1]
    } else {
        if i == costs.len() - 2 {
            costs[i] + count_impl(costs, i + 1)
        } else {
            let cost1 = count_impl(costs, i + 1);
            let cost2 = count_impl(costs, i + 2);
            costs[i] + std::cmp::min(cost1, cost2)
        }
    }
}

fn count(costs: &Vec<u64>) -> u64 {
    let cost1 = count_impl(costs, 1);
    let cost2 = count_impl(costs, 2);
    costs[0] + std::cmp::min(cost1, cost2)
}

fn main() {
    println!("Hello, world!");
}
