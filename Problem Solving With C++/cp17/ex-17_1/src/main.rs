use std::alloc::*;
use std::mem;

struct GenericList<T: Sized> {
    array: *mut T,
    size: usize,
    capacity: usize,
}

impl<T: Sized> GenericList<T> {
    const ALIGN: usize = mem::align_of::<T>();
    const ELEM_SIZE: usize = mem::size_of::<T>();

    fn alloc() -> *mut T {
        unsafe {
            let allocator = System {};
            allocator.alloc(Layout::from_size_align(Self::ELEM_SIZE * 0, Self::ALIGN).unwrap())
                as *mut T
        }
    }

    fn realloc(origin: *mut T, capacity: usize) -> *mut T {
        unsafe {
            let allocator = System {};
            allocator.realloc(
                origin as *mut u8,
                Layout::from_size_align(Self::ELEM_SIZE * 0, Self::ALIGN).unwrap(),
                Self::ELEM_SIZE * capacity,
            ) as *mut T
        }
    }

    fn init(size: usize) -> Self {
        let mut ret = Self {
            array: Self::alloc(),
            size: 0,
            capacity: 0,
        };
        ret
    }

    fn len(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn add(&mut self, new_ele: T) {
        if self.len() < self.capacity() {
            unsafe {
                *self.array.add(self.size) = new_ele;
            }
            self.size += 1;
        }
    }

    fn full(&self) -> bool {
        self.len() == self.capacity()
    }

    fn erase(&mut self) {
        self.size = 0;
    }

    fn begin(&self) -> Iterator<'_, T> {
        Iterator { list: self, pos: 0 }
    }

    fn end(&self) -> Iterator<'_, T> {
        Iterator {
            list: self,
            pos: self.len(),
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
    pos: usize,
}

impl<'a, T: Sized> PartialEq for Iterator<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list && self.pos == other.pos
    }
}

fn main() {
    println!("Hello, world!");
}
