use rand::Rng;
use std::cmp::{Ord, Ordering};
use std::io;

#[derive(PartialEq, Eq)]
enum Point {
    Point(u32),
    Over,
}

enum GameResult {
    PlayerWin,
    HouseWin,
    Draw,
}

struct Game {
    player_points: Vec<u32>,
    house_points: Vec<u32>,
}

impl Game {
    fn new<T: Rng>(rng: &mut T) -> Game {
        let mut player_points = Vec::<u32>::new();
        let mut house_points = Vec::<u32>::new();

        player_points.push(Self::roll(rng));
        house_points.push(Self::roll(rng));

        Game {
            player_points: player_points,
            house_points: house_points,
        }
    }

    fn refresh_points(possible_points: &mut Vec<u32>, points: &[Option<u32>; 2]) {
        if possible_points.is_empty() {
            for point in points {
                if point.is_some() {
                    possible_points.push(point.unwrap());
                }
            }
        } else {
            for _ in 0..possible_points.len() {
                let old_point = possible_points.remove(0);
                for point in points {
                    if point.is_some() {
                        let new_point = old_point + point.unwrap();
                        if new_point <= 21 {
                            possible_points.push(new_point);
                        }
                    }
                }
            }
        }
    }

    fn parse_points(points: &Vec<u32>) -> Point {
        let mut possible_points = Vec::<u32>::new();
        for point in points {
            match *point {
                point if 2 <= point && point <= 10 => {
                    Self::refresh_points(&mut possible_points, &[Option::Some(point), Option::None])
                }
                point if point == 1 => {
                    Self::refresh_points(&mut possible_points, &[Option::Some(1), Option::Some(11)])
                }
                _ => panic!("Invalid point!"),
            }
            if possible_points.is_empty() {
                return Point::Over;
            }
        }
        if !possible_points.is_empty() {
            possible_points.sort();
            possible_points.reverse();
            Point::Point(possible_points[0])
        } else {
            Point::Over
        }
    }

    fn roll<T: Rng>(rng: &mut T) -> u32 {
        let mut dice = rng.gen_range(1..=13);
        if dice > 10 {
            dice = 10;
        }
        dice
    }

    fn player_point(&self) -> Point {
        Self::parse_points(&self.player_points)
    }

    fn house_point(&self) -> Point {
        Self::parse_points(&self.house_points)
    }

    fn player_turn<T: Rng>(&mut self, rng: &mut T) -> bool {
        loop {
            let this_roll = Self::roll(rng);
            println!("Roll: {}", this_roll);
            self.player_points.push(this_roll);
            let player = self.player_point();

            match player {
                Point::Point(point) => {
                    println!("Now player's point: {}", point);
                }
                Point::Over => {
                    println!("Now player's point over 21!");
                    return true;
                }
            }

            println!("Continue? ");
            if !read_confirm() {
                break;
            }
        }
        return false;
    }

    fn house_turn<T: Rng>(&mut self, rng: &mut T) -> bool {
        loop {
            let this_roll = Self::roll(rng);
            println!("Roll: {}", this_roll);
            self.house_points.push(this_roll);
            let house = self.house_point();

            match house {
                Point::Point(point) => {
                    println!("Now house's point: {}", point);
                    if point >= 17 {
                        return false;
                    }
                }
                Point::Over => {
                    println!("Now house's point over 21!");
                    return true;
                }
            }
        }
    }

    fn result(&self) -> GameResult {
        let player = self.player_point();
        let house = self.house_point();
        if let Point::Point(player) = player {
            if let Point::Point(house) = house {
                match player.cmp(&house) {
                    Ordering::Less => GameResult::PlayerWin,
                    Ordering::Equal => GameResult::Draw,
                    Ordering::Greater => GameResult::HouseWin,
                }
            } else {
                GameResult::PlayerWin
            }
        } else {
            match house {
                Point::Point(_) => GameResult::HouseWin,
                Point::Over => GameResult::Draw,
            }
        }
    }
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn read_u64() -> u64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<u64>() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut principal = 100;

    loop {
        println!("Enter this turn wager: ");
        let wager = read_u64();
        if wager > principal {
            println!("Wager greater than")
        } else {
            let mut game = Game::new(&mut rng);

            if game.player_turn(&mut rng) {
                principal -= wager;
                println!("Player lose, now player's principal: {}", principal);
            } else {
                if game.house_turn(&mut rng) {
                    principal += wager;
                    println!("House lose, now player's principal: {}", principal);
                } else {
                    let result = game.result();

                    match result {
                        GameResult::PlayerWin => {
                            principal += wager;
                            println!("House lose, now player's principal: {}", principal);
                        }
                        GameResult::HouseWin => {
                            principal -= wager;
                            println!("Player lose, now player's principal: {}", principal);
                        }
                        GameResult::Draw => {
                            println!("Draw!");
                        }
                    }
                }
            }
        }

        println!("Continue? ");
        if !read_confirm() {
            break;
        }
    }
}
