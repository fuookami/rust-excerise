struct Player {
    name: String,
    point: u64,
}

struct Record {
    players: Vec<Player>,
}

impl Record {
    fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    fn len(&self) -> usize {
        self.players.len()
    }
}

const MAX_LEN: usize = 10;

fn add(record: &mut Record, name: &str, point: u64) {
    if record.is_empty() {
        record.players.push(Player {
            name: name.to_string(),
            point: point,
        });
    } else if record.len() == MAX_LEN && point > record.players.last().unwrap().point {
        for i in 0..record.players.len() {
            if record.players[i].point < point {
                record.players.remove(i);
                record.players.insert(
                    i,
                    Player {
                        name: name.to_string(),
                        point: point,
                    },
                );
                break;
            }
        }
    } else {
        let mut flag = false;
        for i in 0..record.players.len() {
            if record.players[i].point < point {
                record.players.insert(
                    i,
                    Player {
                        name: name.to_string(),
                        point: point,
                    },
                );
                flag = true;
                break;
            }
        }
        if !flag {
            record.players.push(Player {
                name: name.to_string(),
                point: point,
            });
        }
    }
}

fn print(record: &Record) {
    for i in 0..record.players.len() {
        println!(
            "{:2}: {} {}",
            i + 1,
            record.players[i].name,
            record.players[i].point
        );
    }
}

fn print_one(record: &Record, name: &str) {
    let mut flag = false;
    for i in 0..record.players.len() {
        if record.players[i].name == name {
            println!(
                "{:2}: {} {}",
                i + 1,
                record.players[i].name,
                record.players[i].point
            );
            flag = true;
        }
    }
    if !flag {
        println!("No record of {}", name);
    }
}

fn delete(record: &mut Record, name: &str) {
    let mut i = 0;
    while i != record.players.len() {
        if record.players[i].name == name {
            record.players.remove(i);
        } else {
            i += 1;
        }
    }
}

fn main() {
    let mut record = Record {
        players: Vec::new(),
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
