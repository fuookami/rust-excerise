fn insert_sort_impl(arr: &mut Vec<i64>, len: usize) {
    let mut i = 0;
    while i < len {
        if arr[i] > arr[len] {
            break;
        }
        i += 1;
    }
    let temp = arr[len];
    for j in ((i + 1)..(len + 1)).rev() {
        arr[j] = arr[j - 1];
    }
    arr[i] = temp;
}

fn insert_sort(arr: &mut Vec<i64>, pred: fn(&Vec<i64>)) {
    for i in 1..arr.len() {
        insert_sort_impl(arr, i);
        pred(arr);
    }
}

fn print(arr: &Vec<i64>) {
    for num in arr {
        print!("{} ", num);
    }
    print!("\n");
}

fn main() {
    insert_sort(&mut vec![8, 6, 10, 2, 16, 4, 18, 14, 12, 10], print);
}
