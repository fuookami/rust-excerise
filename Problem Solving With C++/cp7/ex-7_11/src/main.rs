#[macro_use]
extern crate strum_macros;

use rand::Rng;
use std::fmt;

#[derive(Clone, Copy, AsRefStr, PartialEq, Eq)]
#[repr(u32)]
enum Column {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

struct Seat {
    row: u32,
    column: Column,
    booked: bool,
}

impl Seat {
    fn new(row: u32, column: u32) -> Self {
        Self {
            row: row,
            column: unsafe { std::mem::transmute(column) },
            booked: false,
        }
    }

    fn book(&mut self) {
        self.booked = true;
    }
}

struct Seats {
    seats: Vec<Seat>,
}

impl Seats {
    fn new(row: u32) -> Self {
        let mut seats = Vec::new();
        for i in 0..row {
            for j in 0..4 {
                seats.push(Seat::new(i + 1, j));
            }
        }
        Self { seats: seats }
    }

    fn book(&mut self, row: u32, column: u32) -> bool {
        for seat in &mut self.seats {
            if seat.row == row && seat.column == unsafe { std::mem::transmute(column) } {
                if seat.booked {
                    return false;
                } else {
                    seat.book();
                    return true;
                }
            }
        }
        return false;
    }
}

impl fmt::Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut row = 0;
        for seat in &self.seats {
            if seat.row != row {
                match if seat.row == 1 {
                    write!(f, "{} ", seat.row)
                } else {
                    write!(f, "\n{} ", seat.row)
                } {
                    fmt::Result::Ok(_) => {}
                    fmt::Result::Err(err) => return fmt::Result::Err(err),
                }
                row = seat.row;
            }

            match if !seat.booked {
                write!(f, "{} ", seat.column.as_ref())
            } else {
                write!(f, "X ")
            } {
                fmt::Result::Ok(_) => {}
                fmt::Result::Err(err) => return fmt::Result::Err(err),
            }
        }
        write!(f, "\n")
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut seats = Seats::new(7);
    for _ in 0..10 {
        let row = rng.gen_range(1..8);
        let column = rng.gen_range(0..4);
        println!(
            "Book {}{}",
            row,
            unsafe { std::mem::transmute::<u32, Column>(column) }.as_ref()
        );
        if seats.book(row, column) {
            println!("{}", seats);
        } else {
            println!("Failed booking!");
        }
    }
}
