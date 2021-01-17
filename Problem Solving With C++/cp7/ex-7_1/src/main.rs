use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut fin = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(fin) => fin,
    };

    let mut s = String::new();
    match fin.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    };

    s
}

fn write_file(path: &str, context: &str) {
    let path = Path::new(path);
    let display = path.display();

    let mut fout = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(fout) => fout,
    };

    match fout.write(context.as_bytes()) {
        Err(why) => panic!("couldn't write {}: {}", display, why),
        Ok(_) => {}
    };
}

fn generate_datas() -> String {
    let mut rng = rand::thread_rng();
    let mut values = Vec::<f64>::new();
    for _ in 0..12 {
        values.push(rng.gen_range(0.0..100.0));
    }
    let mut ret = String::new();
    ret.push_str(&format!(
        "{}\n",
        values.iter().fold(0., |acc, x| acc + x) / values.len() as f64
    ));
    for value in values {
        ret.push_str(&format!("{}\n", value));
    }
    ret
}

fn read_datas(context: &str) -> (f64, Vec<f64>) {
    let mut average_value = 0.;
    let mut values = Vec::<f64>::new();
    let mut flag = 0;
    for s in context.split_ascii_whitespace().filter(|s| !s.is_empty()) {
        match flag {
            0 => {
                average_value = match s.parse::<f64>() {
                    Result::Ok(value) if value > 0. => value,
                    _ => panic!("Invalid value!"),
                }
            }
            i if i <= 12 => values.push(match s.parse::<f64>() {
                Result::Ok(value) if value > 0. => value,
                _ => panic!("Invalid value!"),
            }),
            _ => panic!("Invalid input!"),
        }
        flag += 1;
    }
    (average_value, values)
}

fn format_datas(average_value: f64, values: &Vec<f64>) -> String {
    let mut ret = String::new();
    ret.push_str("Month,Value,Average\n");
    for i in 0..values.len() {
        ret.push_str(&format!(
            "{},{:.2},{:.2}\n",
            i + 1,
            values[i],
            average_value
        ));
    }
    ret
}

fn main() {
    write_file("input.txt", &generate_datas());
    let (average_value, values) = read_datas(&read_file("input.txt"));
    write_file("output.csv", &format_datas(average_value, &values));
}
