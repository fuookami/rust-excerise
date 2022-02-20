use std::collections::BTreeSet;

fn delete_repeats<T: Ord>(arr: &mut Vec<T>) {
    unsafe {
        let mut cache = BTreeSet::<&T>::new();
        let mut repeats = BTreeSet::<T>::new();
        let mut i = 0;
        let len = arr.len();
        while i != len {
            let this_ele = arr.as_ptr().add(i);
            if repeats.contains(&*this_ele) {
                arr.remove(i);
            } else if cache.contains(&*this_ele) {
                repeats.insert(arr.remove(i));
            } else {
                cache.insert(&*this_ele);
            }
            i += 1;
        }
    }
}

fn main() {
    let mut arr = vec![1, 1, 2];
    delete_repeats(&mut arr);
    assert_eq!(arr, vec![1, 2]);
}
