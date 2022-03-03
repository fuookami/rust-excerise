struct Map<T: Sized + PartialEq, U: Sized> {
    _array: Vec<(T, U)>,
}

impl<T: Sized + PartialEq, U: Sized> Map<T, U> {
    fn new() -> Map<T, U> {
        Map { _array: vec![] }
    }

    fn insert(&mut self, key: T, value: U) {
        if !self.contains(&key) {
            self._array.push((key, value))
        }
    }

    fn set(&mut self, key: T, value: U) {
        for pair in &mut self._array {
            if pair.0 == key {
                pair.1 = value;
                break;
            }
        }
    }

    fn get(&self, key: T) -> Option<&U> {
        for pair in &self._array {
            if pair.0 == key {
                return Option::Some(&pair.1);
            }
        }
        Option::None
    }

    fn erase(&mut self, key: T) {
        if self.contains(&key) {
            self._array.retain(|(x, _)| x == &key)
        }
    }

    fn contains(&self, key: &T) -> bool {
        self._array.iter().any(|(x, _)| x == key)
    }
}

fn main() {
    println!("Hello, world!");
}
