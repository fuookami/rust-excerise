use std::alloc::*;
use std::cmp;
use std::mem;
use std::ops;

struct Vector<T: Sized> {
    ptr: *mut T,
    _size: usize,
    _capacity: usize,
}

impl<T: Sized> ops::Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, pos: usize) -> &T {
        unsafe { &*self.ptr.wrapping_offset(pos as isize) }
    }
}

impl<T: Sized> ops::IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, pos: usize) -> &mut T {
        unsafe { &mut *self.ptr.wrapping_offset(pos as isize) }
    }
}

impl<T: Sized + PartialEq<T>> cmp::PartialEq<Vector<T>> for Vector<T> {
    fn eq(&self, other: &Vector<T>) -> bool {
        if self._size != other._size {
            false
        } else {
            for i in 0..self._size {
                if self[i] != other[i] {
                    return false;
                }
            }
            true
        }
    }
}

impl<T: Sized> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            let allocator = System {};
            allocator.dealloc(
                self.ptr as *mut u8,
                Layout::from_size_align(Self::ELEM_SIZE * self._capacity, Self::ALIGN).unwrap(),
            );
        }
    }
}

impl<T: Sized + Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        let mut ret = Self::_new();
        ret.reserve(self._capacity);
        for i in 0..self._size {
            ret[i] = self[i].clone();
        }
        ret._size = self._size;
        ret
    }
}

impl<T: Sized> Vector<T> {
    const ALIGN: usize = mem::align_of::<T>();
    const ELEM_SIZE: usize = mem::size_of::<T>();

    fn _new() -> Self {
        return Self {
            ptr: std::ptr::null_mut(),
            _size: 0,
            _capacity: 0,
        };
    }

    fn new_from_value(new_elem: T) -> Self
    where
        T: std::clone::Clone,
    {
        Self::new_from_amount_and_value(50, new_elem)
    }

    fn new_from_amount_and_value(amount: usize, new_elem: T) -> Self
    where
        T: std::clone::Clone,
    {
        let mut ret = Self::_new();
        ret.resize(amount, new_elem);
        ret
    }

    fn push_back(&mut self, new_elem: T) {
        if self._size == self._capacity {
            let new_capacity = self._capacity / 2 * 3 + 1;
            self.reserve(new_capacity);
            let i = self._size;
            self[i] = new_elem;
            self._size += 1;
        }
    }

    fn capacity(&self) -> usize {
        self._capacity
    }

    fn size(&self) -> usize {
        self._size
    }

    fn resize(&mut self, new_size: usize, new_elem: T)
    where
        T: std::clone::Clone,
    {
        unsafe {
            if new_size != self._size {
                let allocator = System {};
                if self.ptr.is_null() {
                    self.ptr = allocator.alloc(
                        Layout::from_size_align(Self::ELEM_SIZE * self._capacity, Self::ALIGN)
                            .unwrap(),
                    ) as *mut T;
                } else {
                    self.ptr = allocator.realloc(
                        self.ptr as *mut u8,
                        Layout::from_size_align(Self::ELEM_SIZE * self._capacity, Self::ALIGN)
                            .unwrap(),
                        Self::ELEM_SIZE * new_size,
                    ) as *mut T;
                }
                if self._size < new_size {
                    for i in self._size..new_size {
                        self[i] = new_elem.clone();
                    }
                }
                self._size = new_size;
                self._capacity = new_size;
            }
        }
    }

    fn reserve(&mut self, new_capacity: usize) {
        unsafe {
            if new_capacity > self._capacity {
                let allocator = System {};
                if self.ptr.is_null() {
                    self.ptr = allocator.alloc(
                        Layout::from_size_align(Self::ELEM_SIZE * self._capacity, Self::ALIGN)
                            .unwrap(),
                    ) as *mut T;
                } else {
                    self.ptr = allocator.realloc(
                        self.ptr as *mut u8,
                        Layout::from_size_align(Self::ELEM_SIZE * self._capacity, Self::ALIGN)
                            .unwrap(),
                        Self::ELEM_SIZE * new_capacity,
                    ) as *mut T;
                }
                self._capacity = new_capacity;
            }
        }
    }
}

fn main() {}
