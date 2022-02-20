use std::alloc::*;
use std::mem;

struct GenericStack<T: Sized> {
    array: *mut T,
    size: usize,
    capacity: usize,
}

impl<T: Sized> GenericStack<T> {
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

    fn empty(&self) -> bool {
        self.len() == 0
    }

    fn push(&mut self, new_ele: T) {
        if self.len() == self.capacity() {
            self.capacity = self.capacity * 3 / 2 + 1;
            Self::realloc(self.array, self.capacity);
        }

        unsafe {
            *self.array.add(self.size) = new_ele;
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> where T: Clone {
        match self.empty() {
            true => Option::None,
            false => {
                self.size -= 1;
                unsafe { Option::Some((*self.array.add(self.size - 1)).clone()) }
            }
        }
    }

    fn top(&self) -> &T {
        unsafe { &*self.array.add(self.size - 1) }
    }
}

fn main() {
    println!("Hello, world!");
}
