use rand::Rng;
use std::io;

#[derive(Clone, Copy)]
enum Life {
    Alive,
    Dead,
}

struct World {
    width: usize,
    height: usize,
    cells: Vec<Life>,
    generation: usize,
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let amount = width * height;
        let mut cells = Vec::new();
        for _ in 0..amount {
            match rng.gen_range(0..2) {
                0 => cells.push(Life::Alive),
                1 => cells.push(Life::Dead),
                _ => panic!("Impossible"),
            }
        }
        Self {
            width: width,
            height: height,
            cells: cells,
            generation: 0,
        }
    }

    fn index(&self, vector: [usize; 2]) -> usize {
        vector[0] * self.width + vector[1]
    }

    fn vector(&self, index: usize) -> [usize; 2] {
        [index / self.width, index % self.width]
    }

    fn alive_neighbour_amount(&self, index: usize) -> usize {
        self.count(index, |life: Life| -> usize {
            match life {
                Life::Alive => 1,
                Life::Dead => 0,
            }
        })
    }

    fn count(&self, index: usize, counter: fn(Life) -> usize) -> usize {
        let mut amount = 0;
        let vector = self.vector(index);
        if vector[0] != 0 {
            amount += counter(self.cells[self.index([vector[0] - 1, vector[1]])]);
            if vector[1] != 0 {
                amount += counter(self.cells[self.index([vector[0] - 1, vector[1] - 1])]);
            }
            if vector[1] != self.width - 1 {
                amount += counter(self.cells[self.index([vector[0] - 1, vector[1] + 1])]);
            }
        }
        if vector[0] != self.height - 1 {
            amount += counter(self.cells[self.index([vector[0] + 1, vector[1]])]);
            if vector[1] != 0 {
                amount += counter(self.cells[self.index([vector[0] + 1, vector[1] - 1])]);
            }
            if vector[1] != self.width - 1 {
                amount += counter(self.cells[self.index([vector[0] + 1, vector[1] + 1])]);
            }
        }
        if vector[1] != 0 {
            amount += counter(self.cells[self.index([vector[0], vector[1] - 1])]);
        }
        if vector[1] != self.width - 1 {
            amount += counter(self.cells[self.index([vector[0], vector[1] + 1])]);
        }
        amount
    }

    fn next_genration(&self) -> Self {
        let amount = self.width * self.height;
        let mut cells = Vec::new();
        for i in 0..amount {
            cells.push(match self.cells[i] {
                Life::Alive => match self.alive_neighbour_amount(i) {
                    0 | 1 => Life::Dead,
                    j if j > 3 => Life::Dead,
                    _ => Life::Alive,
                },
                Life::Dead => match self.alive_neighbour_amount(i) {
                    3 => Life::Alive,
                    _ => Life::Dead,
                },
            })
        }
        Self {
            width: self.width,
            height: self.height,
            cells: cells,
            generation: self.generation + 1,
        }
    }

    fn display(&self) {
        println!("Generation: {}", self.generation);
        for i in 0..self.height {
            for j in 0..self.width {
                match self.cells[self.index([i, j])] {
                    Life::Alive => print!("*"),
                    Life::Dead => print!(" "),
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn main() {
    let mut world = World::new(80, 22);
    world.display();
    loop {
        world = world.next_genration();
        world.display();

        println!("Continue? ");
        if !read_confirm() {
            break;
        }
    }
}
