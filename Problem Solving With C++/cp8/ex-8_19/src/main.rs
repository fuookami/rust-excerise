struct Record {
    names: Vec<String>,
    points: Vec<u64>,
}

impl Record {
    fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    fn len(&self) -> usize {
        self.names.len()
    }
}

const MAX_LEN: usize = 10;

fn add(record: &mut Record, name: &str, point: u64) {
    if record.is_empty() {
        record.names.push(name.to_string());
        record.points.push(point);
    } else if record.len() == MAX_LEN && point > *record.points.last().unwrap() {
        for i in 0..record.points.len() {
            if record.points[i] < point {
                record.names.remove(i);
                record.points.remove(i);
                record.names.insert(i, name.to_string());
                record.points.insert(i, point);
                break;
            }
        }
    } else {
        let mut flag = false;
        for i in 0..record.points.len() {
            if record.points[i] < point {
                record.names.insert(i, name.to_string());
                record.points.insert(i, point);
                flag = true;
                break;
            }
        }
        if !flag {
            record.names.push(name.to_string());
            record.points.push(point);
        }
    }
}

fn print(record: &Record) {
    for i in 0..record.points.len() {
        println!("{:2}: {} {}", i + 1, record.names[i], record.points[i]);
    }
}

fn print_one(record: &Record, name: &str) {
    let mut flag = false;
    for i in 0..record.names.len() {
        if record.names[i] == name {
            println!("{:2}: {} {}", i + 1, record.names[i], record.points[i]);
            flag = true;
        }
    }
    if !flag {
        println!("No record of {}", name);
    }
}

fn delete(record: &mut Record, name: &str) {
    let mut i = 0;
    while i != record.points.len() {
        if record.names[i] == name {
            record.names.remove(i);
            record.points.remove(i);
        } else {
            i += 1;
        }
    }
}

fn main() {
    let mut record = Record {
        names: Vec::new(),
        points: Vec::new(),
    };

    add(&mut record, "Bill", 100);
    add(&mut record, "Bill", 100);
    add(&mut record, "Bill", 99);
    add(&mut record, "Bob", 50);

    print(&record);
    println!("");
    print_one(&record, "Bill");
    println!("");

    delete(&mut record, "Bill");
    print(&record);
    println!("");
    print_one(&record, "Bill");
}
