struct GenericList<T: Sized> {
    array: Vec<Option<T>>,
    size: usize,
}

impl<T: Sized> GenericList<T> {
    fn init(size: usize) -> Self {
        let mut ret = Self {
            array: Vec::new(),
            size: 0,
        };
        for _ in 0..size {
            ret.array.push(Option::None)
        }
        ret
    }

    fn len(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.array.len()
    }

    fn add(&mut self, new_ele: T) {
        if self.len() < self.capacity() {
            self.array[self.size].replace(new_ele);
            self.size += 1;
        }
    }

    fn full(&self) -> bool {
        self.len() == self.capacity()
    }

    fn erase(&mut self) {
        self.array.clear();
        self.size = 0;
    }

    fn begin(&self) -> Iterator<'_, T> {
        Iterator {
            list: self,
            pos: 0
        }
    }

    fn end(&self) -> Iterator<'_, T> {
        Iterator {
            list: self,
            pos: self.len()
        }
    }
}

impl<T: Sized> PartialEq for &GenericList<T> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

struct Iterator<'a, T: Sized> {
    list: &'a GenericList<T>,
    pos: usize
}

impl<'a, T: Sized> PartialEq for Iterator<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list && self.pos == other.pos
    }
}

fn main() {
    println!("Hello, world!");
}
