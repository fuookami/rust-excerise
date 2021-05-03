extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Debug, AsRefStr, EnumString)]
#[repr(u64)]
enum Month {
    Jan = 1,
    Feb = 2,
    Mar = 3,
    Apr = 4,
    May = 5,
    Jun = 6,
    Jul = 7,
    Aug = 8,
    Sep = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12
}

impl Month {
    fn new_from_str(s: &str) -> Month {
        Month::from_str(s).unwrap()
    }

    fn new_from_num(n: u64) -> Month {
        unsafe { std::mem::transmute(n) }
    }

    fn to_str(&self) -> &str {
        self.as_ref()
    }

    fn to_num(&self) -> u64 {
        unsafe { std::mem::transmute::<Month, u64>(*self) }
    }

    fn next(&self) -> Month {
        unsafe { std::mem::transmute(std::mem::transmute::<Month, u64>(*self) + 1) }
    }
}

fn main() {
    assert_eq!(Month::new_from_num(1), Month::new_from_str("Jan"));
    assert_eq!(Month::new_from_num(1).to_str(), "Jan");
    assert_eq!(Month::new_from_str("Jan").to_num(), 1);
    assert_eq!(Month::new_from_str("Jan").next().to_num(), 2);
    assert_eq!(Month::new_from_str("Jan").next().to_str(), "Feb");
}
