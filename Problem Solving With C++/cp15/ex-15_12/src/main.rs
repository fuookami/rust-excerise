trait Queue {
    fn remove(&mut self);
}

struct PriorityQueue {
    elems: Vec<(u64, u64)>,
}

impl Queue for PriorityQueue {
    fn remove(&mut self) {
        if self.elems.is_empty() {
            return;
        }

        let mut index = 0;
        let mut prio = self.elems[0].1;

        for i in 1..self.elems.len() {
            if self.elems[i].1 < prio {
                index = i;
                prio = self.elems[i].1
            }
        }

        self.elems.remove(index);
    }
}

impl PriorityQueue {
    fn add(&mut self, e: u64, p: u64) {
        self.elems.push((e, p));
    }
}

fn main() {
    println!("Hello, world!");
}
