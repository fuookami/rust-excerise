fn generate_full_permutation_impl(
    result: &mut Vec<Vec<u64>>,
    vals: &Vec<u64>,
    curr_permutation: Vec<u64>,
    i: usize,
) {
    if curr_permutation.len() == vals.len() {
        result.push(curr_permutation)
    } else {
        for j in 0..=i {
            let mut this_permutation = curr_permutation.clone();
            if j != i {
                this_permutation.insert(j, vals[i]);
            } else {
                this_permutation.push(vals[i]);
            }
            generate_full_permutation_impl(vals, this_permutation, i + 1);
        }
    }
}

fn generate_full_permutation(vals: &Vec<u64>) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    generate_full_permutation_impl(&mut result, vals, Vec::new(), 0);
    result
}

fn main() {
    generate_full_permutation(&vec![1, 2, 3, 4]);
}
