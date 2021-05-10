fn add_entry(dyn_arr: Vec<String>, size: &mut usize, new_entry: String) -> Vec<String> {
    let mut new_dyn_arr = Vec::<String>::new();
    for entry in dyn_arr {
        new_dyn_arr.push(entry);
    }
    new_dyn_arr.push(new_entry);
    *size += 1;
    new_dyn_arr
}

fn delete_entry(dyn_arr: Vec<String>, size: &mut usize, entry_to_delete: String) -> Vec<String> {
    let mut flag = false;
    for entry in &dyn_arr {
        if entry == &entry_to_delete {
            flag = true;
        }
    }
    if !flag {
        dyn_arr
    } else {
        let mut new_dyn_arr = Vec::new();
        for entry in dyn_arr {
            if &entry != &entry_to_delete {
                new_dyn_arr.push(entry);
            }
        }
        *size -= 1;
        new_dyn_arr
    }
}

fn print(dyn_arr: &Vec<String>) {
    for entry in dyn_arr {
        print!("{}, ", entry);
    }
    println!("");
}

fn main() {
    let mut dyn_arr = Vec::new();

    dyn_arr.push("Bill".to_string());
    dyn_arr.push("Daniel".to_string());
    dyn_arr.push("Adam".to_string());
    dyn_arr.push("Eric".to_string());
    dyn_arr.push("Lawrence".to_string());
    print(&dyn_arr);

    let mut size = dyn_arr.len();
    let dyn_arr = add_entry(dyn_arr, &mut size, "Holo".to_string());
    print(&dyn_arr);

    let dyn_arr = delete_entry(dyn_arr, &mut size, "Daniel".to_string());
    print(&dyn_arr);
}
