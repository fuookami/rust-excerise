use std::fmt::Display;

struct Set<T: Sized + PartialEq> {
    _array: Vec<T>,
}

impl<T: Sized + PartialEq> Set<T> {
    fn new() -> Set<T> {
        Set { _array: vec![] }
    }

    fn insert(&mut self, ele: T) {
        if !self.contains(&ele) {
            self._array.push(ele);
        }
    }

    fn remove(&mut self, ele: T) {
        if self.contains(&ele) {
            self._array.retain(|x| x == &ele);
        }
    }

    fn len(&self) -> usize {
        self._array.len()
    }

    fn contains(&self, ele: &T) -> bool {
        self._array.contains(ele)
    }

    fn to_vec(&self) -> &Vec<T> {
        &self._array
    }
}

fn generate_full_permutation_impl<T: Sized + PartialEq + Clone + Display>(
    vals: &Set<T>,
    curr_permutation: Vec<T>,
    i: usize,
) {
    if curr_permutation.len() == vals.len() {
        for val in curr_permutation {
            print!("{}, ", val);
        }
        print!("\n");
    } else {
        for j in 0..=i {
            let mut this_permutation = curr_permutation.clone();
            if j != i {
                this_permutation.insert(j, vals.to_vec()[i].clone());
            } else {
                this_permutation.push(vals.to_vec()[i].clone());
            }
            generate_full_permutation_impl(vals, this_permutation, i + 1);
        }
    }
}

fn generate_full_permutation<T: Sized + PartialEq + Clone + Display>(vals: &Set<T>) {
    generate_full_permutation_impl(vals, Vec::new(), 0);
}

fn main() {
    let mut set = Set::new();
    set.insert(0);
    set.insert(1);
    set.insert(2);
    generate_full_permutation(&set);
}
