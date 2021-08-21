struct Computer {
    index: usize,
    user: Option<String>,
}

struct Lab {
    index: usize,
    computers: Vec<Computer>,
}

impl Lab {
    fn new(index: usize, computer_amount: usize) -> Lab {
        let mut computers = Vec::new();
        for i in 0..computer_amount {
            computers.push(Computer {
                index: i + 1,
                user: None,
            })
        }
        Lab {
            index: index,
            computers: computers,
        }
    }

    fn print(&self) {
        print!("{}\t", self.index);
        for computer in &self.computers {
            match &computer.user {
                Some(user) => print!("{}: {} ", computer.index, user),
                None => print!("{}: empty ", computer.index),
            }
        }
        println!("");
    }
}

fn login(id: String, lab_index: usize, computer_index: usize, labs: &mut [Lab]) -> bool {
    for lab in labs {
        if lab.index == lab_index {
            for computer in &mut lab.computers {
                if computer.index == computer_index {
                    match &computer.user {
                        Some(_) => return false,
                        None => {
                            computer.user = Some(id);
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn logout(id: String, labs: &mut [Lab]) -> bool {
    for lab in labs {
        for computer in &mut lab.computers {
            match &computer.user {
                Some(user) => {
                    if user == &id {
                        computer.user = None;
                        return true;
                    }
                }
                None => {}
            }
        }
    }
    false
}

fn find(id: String, labs: &[Lab]) -> Option<(usize, usize)> {
    for lab in labs {
        for computer in &lab.computers {
            match &computer.user {
                Some(user) => {
                    if user == &id {
                        return Some((lab.index, computer.index));
                    }
                }
                None => {}
            }
        }
    }
    None
}

fn print(labs: &[Lab]) {
    for lab in labs {
        lab.print();
    }
}

fn main() {
    let mut labs = [
        Lab::new(1, 5),
        Lab::new(2, 6),
        Lab::new(3, 4),
        Lab::new(4, 3),
    ];

    login("99577".to_string(), 4, 1, &mut labs);
    login("12345".to_string(), 1, 2, &mut labs);
    print(&labs);
    println!("");

    match find("12345".to_string(), &labs) {
        Some((lab_index, computer_index)) => println!("{}, {}", lab_index, computer_index),
        None => println!("None"),
    }
    println!("");

    logout("12345".to_string(), &mut labs);
    print(&labs);
    println!("");

    match find("99577".to_string(), &labs) {
        Some((lab_index, computer_index)) => println!("{}, {}", lab_index, computer_index),
        None => println!("None"),
    }

    match find("12345".to_string(), &labs) {
        Some((lab_index, computer_index)) => println!("{}, {}", lab_index, computer_index),
        None => println!("None"),
    }
}
