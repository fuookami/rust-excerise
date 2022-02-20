struct PrioritizedItem<T: Sized> {
    priority: usize,
    ele: T,
}

struct PrioritizedQueue<T: Sized> {
    queue: Vec<PrioritizedItem<T>>,
}

impl<T: Sized> PrioritizedQueue<T> {
    fn add(&mut self, item: PrioritizedItem<T>) {
        let mut insert_pos = Option::None;
        for i in 0..self.queue.len() {
            if self.queue[i].priority > item.priority {
                insert_pos = Option::Some(i);
                break;
            }
        }
        match insert_pos {
            Option::Some(i) => self.queue.insert(i, item),
            Option::None => self.queue.push(item),
        }
    }

    fn remove(&mut self) -> T {
        self.queue.remove(0).ele
    }
}

fn main() {
    println!("Hello, world!");
}
