extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
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

    fn next(&self) -> MonthImpl {
        unsafe { std::mem::transmute(std::mem::transmute::<MonthImpl, u64>(*self) + 1) }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Month {
    s: &'static str,
}

impl Month {
    fn new(s: &'static str) -> Month {
        Month { s: s }
    }

    fn from_num(n: u64) -> Month {
        Self::new(MonthImpl::new_from_num(n).to_str())
    }

    fn to_str(&self) -> &'static str {
        self.s
    }

    fn write<W: Write>(&self, w: &mut W) -> io::Result<usize> {
        w.write(self.to_str().as_bytes())
    }

    fn to_num(&self) -> u64 {
        MonthImpl::new_from_str(self.s).to_num()
    }

    fn next(&self) -> Month {
        Month {
            s: MonthImpl::new_from_str(self.s).next().to_str(),
        }
    }
}

fn main() {
    let path = Path::new("output.txt");
    let display = path.display();
    let mut fout = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(fout) => fout,
    };

    let _ = Month::from_num(1).write(&mut std::io::stdout());
    let _ = Month::from_num(1).write(&mut fout);

    let _ = Month::new("Jan").next().write(&mut std::io::stdout());
    let _ = Month::new("Jan").next().write(&mut fout);
}
