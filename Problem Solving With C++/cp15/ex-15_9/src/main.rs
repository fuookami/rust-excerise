use std::any::Any;

trait Organism {
    fn shift(&mut self, pos: (usize, usize), world: &mut World);
    fn breed(&mut self, pos: (usize, usize), world: &mut World);
    fn as_any(&self) -> &dyn Any;
}

struct World {
    len: usize,
    width: usize,
    current: u64,
    world: Vec<Vec<Option<Box<dyn Organism>>>>,
    _rng: Box<dyn rand::RngCore>,
}

impl World {
    fn random_pos(&mut self, from: (usize, usize)) -> (usize, usize) {
        let d = self._rng.next_u64() % 4;
        match d {
            0 => (from.0 + 1, from.1),
            1 => (from.0 - 1, from.1),
            2 => (from.0, from.1 + 1),
            3 => (from.0, from.1 - 1),
            _ => panic!("impossible"),
        }
    }

    fn shift(&mut self, from: (usize, usize), to: (usize, usize)) {
        if to.0 >= self.len
            || to.0 == std::usize::MAX
            || to.1 >= self.width
            || to.1 == std::usize::MAX
            || self.world[to.0][to.1].is_some()
        {
            return;
        }

        let mut temp = Option::None;
        std::mem::swap(&mut self.world[from.0][from.1], &mut temp);
        std::mem::swap(&mut temp, &mut self.world[to.0][to.1]);
    }

    fn remove(&mut self, pos: (usize, usize)) {
        self.world[pos.0][pos.1] = Option::None;
    }

    fn add(&mut self, pos: (usize, usize), organism: Box<dyn Organism>) {
        if pos.0 >= self.len
            || pos.0 == std::usize::MAX
            || pos.1 >= self.width
            || pos.1 == std::usize::MAX
            || self.world[pos.0][pos.1].is_some()
        {
            return;
        }

        self.world[pos.0][pos.1] = Option::Some(organism)
    }

    fn get(&mut self, pos: (usize, usize)) -> Option<&dyn Organism> {
        if pos.0 >= self.len
            || pos.0 == std::usize::MAX
            || pos.1 >= self.width
            || pos.1 == std::usize::MAX
        {
            Option::None
        } else {
            match &self.world[pos.0][pos.1] {
                Some(organism) => Option::Some(organism.as_ref()),
                None => Option::None,
            }
        }
    }

    fn next(&mut self) {
        self.current += 1;
        for i in 0..self.len {
            for j in 0..self.width {
                if let Some(organism) = &self.world[i][j] {
                    if let Some(doodlebug) = organism.as_any().downcast_ref::<Doodlebug>() {
                        if doodlebug.born != self.current {
                            // breed
                        }
                    }
                }
            }
        }
        for i in 0..self.len {
            for j in 0..self.width {
                if let Some(organism) = &self.world[i][j] {
                    if let Some(ant) = organism.as_any().downcast_ref::<Ant>() {
                        if ant.born != self.current {
                            // breed
                        }
                    }
                }
            }
        }
        for i in 0..self.len {
            for j in 0..self.width {
                if let Some(organism) = &self.world[i][j] {
                    if let Some(doodlebug) = organism.as_any().downcast_ref::<Doodlebug>() {
                        if doodlebug.born != self.current {
                            // shift
                        }
                    }
                }
            }
        }
        for i in 0..self.len {
            for j in 0..self.width {
                if let Some(organism) = &self.world[i][j] {
                    if let Some(ant) = organism.as_any().downcast_ref::<Ant>() {
                        if ant.born != self.current {
                            // shift
                        }
                    }
                }
            }
        }
    }
}

struct Ant {
    born: u64,
    breeding_cycle: u64,
}

impl Organism for Ant {
    fn shift(&mut self, pos: (usize, usize), world: &mut World) {
        let to = world.random_pos(pos);
        world.shift(pos, to);
    }

    fn breed(&mut self, pos: (usize, usize), world: &mut World) {
        self.breeding_cycle += 1;

        if self.breeding_cycle >= 3 {
            let to = world.random_pos(pos);
            world.add(
                to,
                Box::new(Ant {
                    born: world.current,
                    breeding_cycle: 0,
                }),
            )
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Doodlebug {
    born: u64,
    breeding_cycle: u64,
    eating_cycle: u64,
}

impl Organism for Doodlebug {
    fn shift(&mut self, pos: (usize, usize), world: &mut World) {
        self.eating_cycle += 1;

        if self.eating_cycle >= 3 {
            world.remove(pos);
        } else {
            let next_pos = vec![
                (pos.0 + 1, pos.1),
                (pos.0 - 1, pos.1),
                (pos.0, pos.1 + 1),
                (pos.0, pos.1 - 1),
            ];

            let mut flag = false;
            for pos in next_pos {
                if let Some(organism) = world.get(pos) {
                    if let Some(_) = organism.as_any().downcast_ref::<Ant>() {
                        world.remove(pos);
                        world.shift(pos, pos);

                        flag = true;
                        break;
                    }
                }
            }

            if !flag {
                let to = world.random_pos(pos);
                world.shift(pos, to);
            }
        }
    }

    fn breed(&mut self, pos: (usize, usize), world: &mut World) {
        self.breeding_cycle += 1;

        if self.breeding_cycle >= 8 {
            let to = world.random_pos(pos);
            world.add(
                to,
                Box::new(Doodlebug {
                    born: world.current,
                    breeding_cycle: 0,
                    eating_cycle: 0,
                }),
            )
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    println!("Hello, world!");
}
