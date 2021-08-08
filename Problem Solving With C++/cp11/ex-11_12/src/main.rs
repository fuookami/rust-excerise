#[macro_use]
extern crate strum_macros;

use std::collections::HashMap;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, AsRefStr, PartialEq, Eq, Hash, Clone, Copy)]
enum MayorVote {
    A,
    B,
    C,
}

#[derive(EnumIter, AsRefStr, PartialEq, Eq, Hash, Clone, Copy)]
enum Proposition17Vote {
    D,
    E,
}

#[derive(EnumIter, AsRefStr, PartialEq, Eq, Hash, Clone, Copy)]
enum Measure1Vote {
    F,
    G,
}

#[derive(EnumIter, AsRefStr, PartialEq, Eq, Hash, Clone, Copy)]
enum Measure2Vote {
    H,
    I,
}

struct Voter {
    id: u64,
    vote: (MayorVote, Proposition17Vote, Measure1Vote, Measure2Vote),
}

impl Voter {
    fn new(
        id: u64,
        mayor_vote: MayorVote,
        proposition_17_vote: Proposition17Vote,
        measure_1_vote: Measure1Vote,
        measure_2_vote: Measure2Vote,
    ) -> Self {
        Self {
            id: id,
            vote: (
                mayor_vote,
                proposition_17_vote,
                measure_1_vote,
                measure_2_vote,
            ),
        }
    }
}

impl fmt::Display for Voter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}{}{}{}",
            self.id,
            self.vote.0.as_ref(),
            self.vote.1.as_ref(),
            self.vote.2.as_ref(),
            self.vote.3.as_ref()
        )
    }
}

fn count_mayor_vote_percent(voters: &Vec<Voter>) -> HashMap<MayorVote, f64> {
    let mut counter = HashMap::new();
    for mayor_vote in MayorVote::iter() {
        counter.insert(mayor_vote, 0 as usize);
    }
    for voter in voters {
        *counter.get_mut(&voter.vote.0).unwrap() += 1;
    }

    let mut ret = HashMap::new();
    for mayor_vote in MayorVote::iter() {
        match counter.get(&mayor_vote) {
            Option::Some(amount) => {
                ret.insert(mayor_vote, *amount as f64 / voters.len() as f64);
            }
            Option::None => {
                ret.insert(mayor_vote, 0.);
            }
        }
    }
    ret
}

fn count_proposition_17_percent(voters: &Vec<Voter>) -> HashMap<Proposition17Vote, f64> {
    let mut counter = HashMap::new();
    for proposition_17 in Proposition17Vote::iter() {
        counter.insert(proposition_17, 0 as usize);
    }
    for voter in voters {
        *counter.get_mut(&voter.vote.1).unwrap() += 1;
    }

    let mut ret = HashMap::new();
    for proposition_17 in Proposition17Vote::iter() {
        match counter.get(&proposition_17) {
            Option::Some(amount) => {
                ret.insert(proposition_17, *amount as f64 / voters.len() as f64);
            }
            Option::None => {
                ret.insert(proposition_17, 0.);
            }
        }
    }
    ret
}

fn count_measure_1_percent(voters: &Vec<Voter>) -> HashMap<Measure1Vote, f64> {
    let mut counter = HashMap::new();
    for measure_1 in Measure1Vote::iter() {
        counter.insert(measure_1, 0 as usize);
    }
    for voter in voters {
        *counter.get_mut(&voter.vote.2).unwrap() += 1;
    }

    let mut ret = HashMap::new();
    for measure_1 in Measure1Vote::iter() {
        match counter.get(&measure_1) {
            Option::Some(amount) => {
                ret.insert(measure_1, *amount as f64 / voters.len() as f64);
            }
            Option::None => {
                ret.insert(measure_1, 0.);
            }
        }
    }
    ret
}

fn count_measure_2_percent(voters: &Vec<Voter>) -> HashMap<Measure2Vote, f64> {
    let mut counter = HashMap::new();
    for measure_2 in Measure2Vote::iter() {
        counter.insert(measure_2, 0 as usize);
    }
    for voter in voters {
        *counter.get_mut(&voter.vote.3).unwrap() += 1;
    }

    let mut ret = HashMap::new();
    for measure_2 in Measure2Vote::iter() {
        match counter.get(&measure_2) {
            Option::Some(amount) => {
                ret.insert(measure_2, *amount as f64 / voters.len() as f64);
            }
            Option::None => {
                ret.insert(measure_2, 0.);
            }
        }
    }
    ret
}

fn main() {
    println!("Hello, world!");
}
