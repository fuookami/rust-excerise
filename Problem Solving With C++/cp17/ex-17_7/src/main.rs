struct Set<T: Sized + PartialEq> {
    _array: Vec<T>,
}

impl<T: Sized + PartialEq> Set<T> {
    fn new() -> Set<T> {
        Set { _array: vec![] }
    }

    fn insert(&mut self, ele: T) {
        if !self.contains(&ele) {
            self._array.push(ele);
        }
    }

    fn remove(&mut self, ele: T) {
        if self.contains(&ele) {
            self._array.retain(|x| x == &ele);
        }
    }

    fn len(&self) -> usize {
        self._array.len()
    }

    fn contains(&self, ele: &T) -> bool {
        self._array.contains(ele)
    }

    fn to_vec(&self) -> &Vec<T> {
        &self._array
    }
}

fn main() {
    println!("Hello, world!");
}
