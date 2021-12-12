use rand::Rng;
use std::io;

struct Game {
    _target: u64,
    _min: u64,
    _max: u64,
    _finish: bool,
}

impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            _target: rng.gen::<u64>() % 100,
            _min: 0,
            _max: 100,
            _finish: false,
        }
    }

    fn min(&self) -> u64 {
        self._min
    }

    fn max(&self) -> u64 {
        self._max
    }

    fn guess(&mut self, target: u64) -> bool {
        if self._finish {
            false
        } else if self._min < target && target < self._target {
            self._min = target;
            false
        } else if self._target < target && self._target < self._max {
            self._max = target;
            false
        } else if target == self._target {
            self._finish = true;
            true
        } else {
            false
        }
    }
}

trait Player {
    fn guess(&self, game: &mut Game) -> bool;
}

struct HumanPlayer {}
struct ComputerPlayer {}

impl Player for HumanPlayer {
    fn guess(&self, game: &mut Game) -> bool {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line!");
        game.guess(line.trim().parse::<u64>().unwrap())
    }
}

impl Player for ComputerPlayer {
    fn guess(&self, game: &mut Game) -> bool {
        let mut rng = rand::thread_rng();
        game.guess(game.min() + rng.gen::<u64>() % (game.max() - game.min()))
    }
}

fn main() {
    println!("Hello, world!");
}
