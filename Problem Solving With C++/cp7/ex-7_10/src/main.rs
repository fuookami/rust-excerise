#![feature(vec_remove_item)]
#![warn(deprecated)]

use rand::Rng;
use std::fmt;

const DISH_LEN: usize = 9;

#[derive(PartialEq, Eq)]
enum DishPosition {
    Player1,
    Player2,
    None(u8),
}

impl fmt::Display for DishPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DishPosition::Player1 => write!(f, "X"),
            DishPosition::Player2 => write!(f, "O"),
            DishPosition::None(num) => write!(f, "{}", num),
        }
    }
}

struct Dish {
    positions: [DishPosition; DISH_LEN],
    rest_position: Vec<usize>,
}

impl Dish {
    fn new() -> Dish {
        let mut positions: [DishPosition; DISH_LEN] = unsafe { std::mem::zeroed() };
        let mut rest_position = Vec::new();
        for i in 0..DISH_LEN {
            positions[i] = DishPosition::None(i as u8);
            rest_position.push(i);
        }
        Dish {
            positions: positions,
            rest_position: rest_position,
        }
    }

    fn empty(&self) -> bool {
        self.rest_position.is_empty()
    }

    fn player1_select(&mut self, pos: usize) {
        if let DishPosition::None(_) = self.positions.get(pos).unwrap() {
            self.positions[pos] = DishPosition::Player1;
            self.rest_position.retain(|&x| x != pos);
        } else {
            panic!("Not Empty Position!");
        }
    }

    fn player2_select(&mut self, pos: usize) {
        if let DishPosition::None(_) = self.positions.get(pos).unwrap() {
            self.positions[pos] = DishPosition::Player2;
            self.rest_position.retain(|&x| x != pos);
        } else {
            panic!("Not Empty Position!");
        }
    }
}

impl fmt::Display for Dish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..DISH_LEN {
            if let fmt::Result::Err(err) = write!(f, "{} ", self.positions[i]) {
                return fmt::Result::Err(err);
            }
            if i % 3 == 2 {
                if let fmt::Result::Err(err) = write!(f, "\n") {
                    return fmt::Result::Err(err);
                }
            }
        }
        fmt::Result::Ok(())
    }
}

enum Result {
    Player1Win,
    Player2Win,
    Draw,
}

fn check_one(dish: &Dish, positions: &[usize]) -> Result {
    let mut player1_counter = 0;
    let mut player2_counter = 0;
    for pos in positions {
        if let DishPosition::Player1 = dish.positions.get(*pos).unwrap() {
            player1_counter += 1;
        } else if let DishPosition::Player2 = dish.positions.get(*pos).unwrap() {
            player2_counter += 1;
        }
    }
    if player1_counter == 3 {
        Result::Player1Win
    } else if player2_counter == 3 {
        Result::Player2Win
    } else {
        Result::Draw
    }
}

fn check(dish: &Dish) -> Result {
    static POSITIONS: [[usize; 3]; 8] = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]];
    for pos in &POSITIONS {
        match check_one(dish, pos) {
            Result::Player1Win => return Result::Player1Win,
            Result::Player2Win => return Result::Player2Win,
            Result::Draw => continue,
        };
    }
    Result::Draw
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut dish = Dish::new();
    println!("{}", dish);

    let mut result = Result::Draw;
    while !dish.empty() {
        let player1_select = dish.rest_position[rng.gen_range(0..dish.rest_position.len())];
        dish.player1_select(player1_select);
        println!("{}", dish);
        if let Result::Player1Win = check(&dish) {
            result = Result::Player1Win;
            break;
        }

        if dish.empty() {
            break;
        }

        let player2_select = dish.rest_position[rng.gen_range(0..dish.rest_position.len())];
        dish.player2_select(player2_select);
        println!("{}", dish);
        if let Result::Player2Win = check(&dish) {
            result = Result::Player2Win;
            break;
        }
    }
    match result {
        Result::Player1Win => println!("Player1 Win"),
        Result::Player2Win => println!("Player2 Win"),
        Result::Draw => println!("Draw!"),
    }
}
