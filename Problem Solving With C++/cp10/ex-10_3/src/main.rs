extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Debug, AsRefStr, AsStaticStr, IntoStaticStr, EnumString)]
#[repr(u64)]
enum MonthImpl {
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
    Dec = 12,
}

impl MonthImpl {
    fn new_from_str(s: &str) -> MonthImpl {
        MonthImpl::from_str(s).unwrap()
    }

    fn new_from_num(n: u64) -> MonthImpl {
        unsafe { std::mem::transmute(n) }
    }

    fn to_str(&self) -> &'static str {
        self.into()
    }

    fn to_num(&self) -> u64 {
        unsafe { std::mem::transmute::<MonthImpl, u64>(*self) }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Month {
    num: u64,
}

impl Month {
    fn new(num: u64) -> Month {
        Month { num: num }
    }

    fn from_str(s: &str) -> Month {
        Self::new(MonthImpl::new_from_str(s).to_num())
    }

    fn to_str(&self) -> &'static str {
        MonthImpl::new_from_num(self.num).to_str()
    }

    fn to_num(&self) -> u64 {
        self.num
    }

    fn next(&self) -> Month {
        Month { num: self.num + 1 }
    }
}

fn main() {
    assert_eq!(Month::new(1), Month::from_str("Jan"));
    assert_eq!(Month::new(1).to_str(), "Jan");
    assert_eq!(Month::from_str("Jan").to_num(), 1);
    assert_eq!(Month::from_str("Jan").next().to_num(), 2);
    assert_eq!(Month::from_str("Jan").next().to_str(), "Feb");
}
