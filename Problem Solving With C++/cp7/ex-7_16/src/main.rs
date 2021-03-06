use rand::Rng;

use std::io;

fn shuffle<T>(slice: &mut Vec<T>) {
    let mut rng = rand::thread_rng();
    let len = slice.len();
    for i in 0..len {
        slice.swap(i, rng.gen_range(i..len));
    }
}

struct Position {
    i: usize,
    j: usize,
    point: u64,
    showed: bool,
}

impl Position {
    fn new(i: usize, j: usize, point: u64) -> Self {
        Self {
            i: i,
            j: j,
            point: point,
            showed: false,
        }
    }

    fn show(&mut self) {
        self.showed = true;
    }

    fn hide(&mut self) {
        self.showed = false;
    }
}

struct Game {
    width: usize,
    height: usize,
    positions: Vec<Position>,
}

impl Game {
    fn new(width: usize, height: usize) -> Self {
        let amount = width * height;
        let mut points = Vec::new();
        for i in 0..amount {
            points.push((i / 2) as u64 + 1);
        }
        shuffle(&mut points);

        let mut positions = Vec::new();
        for i in 0..height {
            for j in 0..width {
                positions.push(Position::new(i, j, points[i * width + j]));
            }
        }
        Self {
            width: width,
            height: height,
            positions: positions,
        }
    }

    fn display(&self) {
        print!("  ");
        for i in 0..self.width {
            print!("{} ", i + 1);
        }
        print!("\n");

        let mut k = 0;
        for i in 0..self.height {
            print!("{} ", i + 1);
            for _ in 0..self.width {
                if self.positions[k].showed {
                    print!("{} ", self.positions[k].point);
                } else {
                    print!("* ");
                }
                k += 1;
            }
            print!("\n");
        }
    }

    fn select(&mut self, loc1: (usize, usize), loc2: (usize, usize)) -> bool {
        let mut pos1 = Option::<usize>::None;
        let mut pos2 = Option::<usize>::None;
        for i in 0..self.positions.len() {
            if !self.positions[i].showed
                && self.positions[i].i == loc1.0
                && self.positions[i].j == loc1.1
            {
                pos1 = Option::Some(i);
            }
            if !self.positions[i].showed
                && self.positions[i].i == loc2.0
                && self.positions[i].j == loc2.1
            {
                pos2 = Option::Some(i);
            }
        }

        if pos1.is_none() || pos2.is_none() {
            return false;
        }

        self.positions[pos1.unwrap()].show();
        self.positions[pos2.unwrap()].show();
        self.display();

        if self.positions[pos1.unwrap()].point != self.positions[pos2.unwrap()].point {
            self.positions[pos1.unwrap()].hide();
            self.positions[pos2.unwrap()].hide();
            self.display();
        }

        true
    }

    fn finished(&self) -> bool {
        for pos in &self.positions {
            if !pos.showed {
                return false;
            }
        }
        true
    }
}

fn read_locs() -> Vec<usize> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let mut ret = Vec::new();
    for s in line.trim().split_ascii_whitespace() {
        ret.push(match s.parse::<usize>() {
            Result::Ok(value) => value - 1,
            Result::Err(_) => panic!("Failed to parse!"),
        })
    }
    ret
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn main() {
    let mut game = Game::new(4, 4);
    game.display();

    while !game.finished() {
        println!("Enter locs: ");
        let locs = read_locs();
        if locs.len() != 4 {
            println!("Invalid location!");
            continue;
        }

        if !game.select((locs[0], locs[1]), (locs[2], locs[3])) {
            println!("Invalid location!");
            continue;
        }
        println!("Continue?");
        if !read_confirm() {
            break;
        }
    }
}
