use std::fmt;

const MAX_LIST_SIZE: usize = 50;

struct List {
    list: [f64; MAX_LIST_SIZE],
    size: usize,
}

impl List {
    fn new() -> Self {
        Self {
            list: unsafe { std::mem::zeroed() },
            size: 0,
        }
    }

    fn add(&mut self, value: f64) {
        if self.size != MAX_LIST_SIZE {
            self.list[self.size] = value;
            self.size += 1;
        }
    }

    fn get_last(&self) -> Option<f64> {
        if self.size == 0 {
            Option::None
        } else {
            Option::Some(self.list[self.size - 1])
        }
    }

    fn delete_last(&mut self) {
        if self.size != 0 {
            self.list[self.size - 1] = unsafe { std::mem::zeroed() };
            self.size -= 1;
        }
    }

    fn full(&self) -> bool {
        self.size == MAX_LIST_SIZE
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.size {
            match write!(f, "{}, ", self.list[i]) {
                fmt::Result::Ok(_) => {}
                fmt::Result::Err(err) => return fmt::Result::Err(err),
            }
        }
        fmt::Result::Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
