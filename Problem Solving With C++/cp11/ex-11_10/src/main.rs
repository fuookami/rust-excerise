use std::cmp::Ordering;

#[derive(Clone)]
struct StringSet {
    strs: Vec<String>,
}

impl StringSet {
    fn new() -> Self {
        Self { strs: Vec::new() }
    }

    fn new_from_list(mut list: Vec<String>) -> Self {
        list.sort();
        Self { strs: list }
    }

    fn insert(&mut self, s: String) {
        let mut insert_pos = Option::None;
        for i in 0..self.strs.len() {
            match self.strs[i].cmp(&s) {
                Ordering::Greater => {
                    insert_pos = Option::Some(i);
                    break;
                }
                Ordering::Equal => {
                    break;
                }
                _ => {}
            }
        }
        if let Option::Some(pos) = insert_pos {
            self.strs.insert(pos, s);
        }
    }

    fn remove(&mut self, s: String) {
        for i in 0..self.strs.len() {
            if self.strs[i] == s {
                self.strs.remove(i);
                break;
            }
        }
    }

    fn clear(&mut self) {
        self.strs.clear();
    }

    fn size(&self) -> usize {
        self.size()
    }

    fn sim(&self, rhs: &Self) -> f64 {
        (self._same_element_size(rhs) as f64)
            / ((self.size() as f64).sqrt() * (rhs.size() as f64).sqrt())
    }

    fn _same_element_size(&self, rhs: &Self) -> usize {
        let mut i = 0;
        let mut j = 0;
        let mut counter = 0;
        while i != self.size() && j != rhs.size() {
            match self.strs[i].cmp(&rhs.strs[i]) {
                Ordering::Greater => {
                    j += 1;
                }
                Ordering::Equal => {
                    counter += 1;
                    i += 1;
                    j += 1;
                }
                Ordering::Less => {
                    i += 1;
                }
            }
        }
        counter
    }
}

impl std::ops::Add for StringSet {
    type Output = Self;

    fn add(self, mut rhs: Self) -> Self {
        let mut ret = self;
        for i in 0..rhs.size() {
            let mut s = String::new();
            std::mem::swap(&mut s, &mut rhs.strs[i]);
            ret.insert(s);
        }
        ret
    }
}

impl std::ops::Mul for StringSet {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self {
        let mut ret = StringSet::new();
        let mut i = 0;
        let mut j = 0;
        while i != self.size() && j != rhs.size() {
            match self.strs[i].cmp(&rhs.strs[i]) {
                Ordering::Greater => {
                    j += 1;
                }
                Ordering::Equal => {
                    let mut s = String::new();
                    std::mem::swap(&mut s, &mut self.strs[i]);
                    ret.insert(s);
                    i += 1;
                    j += 1;
                }
                Ordering::Less => {
                    i += 1;
                }
            }
        }
        ret
    }
}

fn main() {
    println!("Hello, world!");
}
