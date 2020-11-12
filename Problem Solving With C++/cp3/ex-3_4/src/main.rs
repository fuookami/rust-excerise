use std::io;

enum Point {
    Point(i32),
    Over,
}

fn refresh_points(possible_points: &mut Vec<i32>, points: &'static [Option<i32>; 2]) {
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

fn parse_points(points: Vec<&str>) -> Option<Point> {
    let mut possible_points = Vec::<i32>::new();
    for point in points {
        match point {
            _ if point == "2" => {
                refresh_points(&mut possible_points, &[Option::Some(2), Option::None])
            }
            _ if point == "3" => {
                refresh_points(&mut possible_points, &[Option::Some(3), Option::None])
            }
            _ if point == "4" => {
                refresh_points(&mut possible_points, &[Option::Some(4), Option::None])
            }
            _ if point == "5" => {
                refresh_points(&mut possible_points, &[Option::Some(5), Option::None])
            }
            _ if point == "6" => {
                refresh_points(&mut possible_points, &[Option::Some(6), Option::None])
            }
            _ if point == "7" => {
                refresh_points(&mut possible_points, &[Option::Some(7), Option::None])
            }
            _ if point == "8" => {
                refresh_points(&mut possible_points, &[Option::Some(8), Option::None])
            }
            _ if point == "9" => {
                refresh_points(&mut possible_points, &[Option::Some(9), Option::None])
            }
            _ if point.to_lowercase() == "t" => {
                refresh_points(&mut possible_points, &[Option::Some(10), Option::None])
            }
            _ if point.to_lowercase() == "j" => {
                refresh_points(&mut possible_points, &[Option::Some(10), Option::None])
            }
            _ if point.to_lowercase() == "q" => {
                refresh_points(&mut possible_points, &[Option::Some(10), Option::None])
            }
            _ if point.to_lowercase() == "k" => {
                refresh_points(&mut possible_points, &[Option::Some(10), Option::None])
            }
            _ if point.to_lowercase() == "a" => {
                refresh_points(&mut possible_points, &[Option::Some(1), Option::Some(11)])
            }
            _ => return Option::None,
        }
        if possible_points.is_empty() {
            return Option::Some(Point::Over);
        }
    }
    if !possible_points.is_empty() {
        possible_points.sort();
        possible_points.reverse();
        Option::Some(Point::Point(possible_points[0]))
    } else {
        Option::Some(Point::Over)
    }
}

fn read_points() -> Option<Point> {
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return Option::None;
    }
    parse_points(line.trim().split(" ").collect::<Vec<&str>>())
}

fn main() {
    loop {
        println!("Enter points: ");
        let points = read_points();
        match points {
            Option::Some(point) => match point {
                Point::Point(point) => println!("Point: {}", point),
                Point::Over => println!("Over!"),
            },
            Option::None => break,
        };
    }
}
