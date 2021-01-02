use rand::Rng;
use std::collections::HashMap;

static SHOORTER_SEQUENCE: [(Puzzler, f64); 3] = [
    (Puzzler::Aaron, 0.1 / 0.3),
    (Puzzler::Bob, 0.5),
    (Puzzler::Charlie, 1.0),
];

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Puzzler {
    Aaron,
    Bob,
    Charlie,
}

struct Duel {
    alive: HashMap<Puzzler, bool>,
}

impl Duel {
    fn new() -> Duel {
        let mut alive = HashMap::<Puzzler, bool>::new();
        for (puzzler, _) in SHOORTER_SEQUENCE.iter() {
            alive.insert(*puzzler, false);
        }

        Duel { alive: alive }
    }

    fn shoot<T: Rng>(rng: &mut T, accuracy: f64) -> bool {
        rng.gen_range(0.0..1.0) > accuracy
    }

    fn alive(&self, puzzler: Puzzler) -> bool {
        *self.alive.get(&puzzler).unwrap()
    }

    fn alive_best_shoorter(&self, shoorter: Puzzler) -> Option<Puzzler> {
        static SHOORTER_RANK: [Puzzler; 3] = [Puzzler::Charlie, Puzzler::Bob, Puzzler::Aaron];

        for i in 0..SHOORTER_RANK.len() {
            let puzzler = SHOORTER_RANK[i];
            if puzzler != shoorter && self.alive(puzzler) {
                return Option::Some(puzzler);
            }
        }
        Option::None
    }

    fn only_one_alive(&self) -> bool {
        self.alive.values().filter(|v| **v).count() == 1
    }

    fn duel1(&mut self) -> Puzzler {
        let mut rng = rand::thread_rng();
        while self.only_one_alive() {
            for i in 0..SHOORTER_SEQUENCE.len() {
                let (shoorter, accuracy) = SHOORTER_SEQUENCE[i];
                let target = self.alive_best_shoorter(shoorter);
                match target {
                    Option::Some(target) => {
                        if Self::shoot(&mut rng, accuracy) {
                            self.alive.entry(target).and_modify(|flag| *flag = false);
                        }
                    }
                    Option::None => return shoorter,
                }
            }
        }
        *self.alive.iter().nth(0).unwrap().0
    }

    fn duel2(&mut self) -> Puzzler {
        let mut rng = rand::thread_rng();
        let mut flag = false;
        while self.only_one_alive() {
            for i in 0..SHOORTER_SEQUENCE.len() {
                let (shoorter, accuracy) = SHOORTER_SEQUENCE[i];
                if shoorter == Puzzler::Aaron && flag {
                    flag = true;
                    continue;
                }

                let target = self.alive_best_shoorter(shoorter);
                match target {
                    Option::Some(target) => {
                        if Self::shoot(&mut rng, accuracy) {
                            self.alive.entry(target).and_modify(|flag| *flag = false);
                        }
                    }
                    Option::None => return shoorter,
                }
            }
        }
        *self.alive.iter().nth(0).unwrap().0
    }
}

fn main() {
    {
        println!("Duel1: ");
        let mut duel_winner = HashMap::<Puzzler, u64>::new();
        for (puzzler, _) in SHOORTER_SEQUENCE.iter() {
            duel_winner.insert(*puzzler, 0);
        }
        for _ in 0..1000 {
            let winner = Duel::new().duel1();
            duel_winner.entry(winner).and_modify(|counter| {
                *counter += 1;
            });
        }

        for (k, v) in duel_winner.iter() {
            println!("{:?} win {} times", k, v);
        }
        println!("");
    }
    {
        println!("Duel2: ");
        let mut duel_winner = HashMap::<Puzzler, u64>::new();
        for (puzzler, _) in SHOORTER_SEQUENCE.iter() {
            duel_winner.insert(*puzzler, 0);
        }
        for _ in 0..1000 {
            let winner = Duel::new().duel2();
            duel_winner.entry(winner).and_modify(|counter| {
                *counter += 1;
            });
        }

        for (k, v) in duel_winner.iter() {
            println!("{:?} win {} times", k, v);
        }
    }
}
