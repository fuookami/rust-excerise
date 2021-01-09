use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Scores {
    family_name: String,
    name: String,
    scores: Vec<f64>,
}

impl Scores {
    fn averate_scores(&self) -> f64 {
        self.scores.iter().sum::<f64>() / self.scores.len() as f64
    }

    fn to_string(&self) -> String {
        let mut ret = String::new();
        ret.push_str(&format!("{} {} ", self.family_name, self.name));
        for score in &self.scores {
            ret.push_str(&format!("{:.0} ", score));
        }
        ret.push_str(&format!("{:.1}", self.averate_scores()));
        ret
    }

    fn from_string(context: &str) -> Scores {
        let mut family_name = String::new();
        let mut name = String::new();
        let mut scores = Vec::<f64>::new();
        let mut flag = 0;

        for s in context.split(' ') {
            match flag {
                0 => family_name = String::from(s),
                1 => name = String::from(s),
                _ if flag < 12 => scores.push(match s.parse::<f64>() {
                    Ok(score) => score,
                    Err(_) => panic!("Fail to parse score!"),
                }),
                _ => panic!("Invalid scores!"),
            }
            flag += 1;
        }

        return Scores {
            family_name: family_name,
            name: name,
            scores: scores,
        };
    }
}

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

    return s;
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

fn read_scores(context: &str) -> Vec<Scores> {
    let mut ret = Vec::<Scores>::new();
    for line in context
        .split(|c| c == '\n' || c == '\r')
        .filter(|s| !s.is_empty())
    {
        ret.push(Scores::from_string(line));
    }
    ret
}

fn wirte_scores(scores: &Vec<Scores>) -> String {
    let mut ret = String::new();
    for score in scores {
        ret.push_str(&format!("{}\n", score.to_string()));
    }
    return ret;
}

fn main() {
    write_file(
        "output.txt",
        &wirte_scores(&read_scores(&read_file("input.txt"))),
    );
}
