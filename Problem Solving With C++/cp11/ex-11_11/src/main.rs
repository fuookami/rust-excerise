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
        ret._reserve(self._capacity);
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

    fn size(&self) -> usize {
        self._size
    }

    fn add_entry(&mut self, new_elem: T) {
        if self._size == self._capacity {
            let new_capacity = self._capacity / 2 * 3 + 1;
            self._reserve(new_capacity);
            let i = self._size;
            self[i] = new_elem;
            self._size += 1;
        }
    }

    fn delete_entry(&mut self, elem_to_del: &T)
    where
        T: cmp::PartialEq,
    {
        let mut pos = Option::None;
        for i in 0..self._size {
            if &self[i] == elem_to_del {
                pos = Option::Some(i);
                break;
            }
        }
        if let Option::Some(del_pos) = pos {
            for i in del_pos..(self._size) - 1 {
                mem::swap(
                    unsafe { &mut *self.ptr.wrapping_offset(i as isize) },
                    unsafe { &mut *self.ptr.wrapping_offset((i + 1) as isize) },
                );
            }
            self._size -= 1;
        }
    }

    fn _reserve(&mut self, new_capacity: usize) {
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

fn main() {
    println!("Hello, world!");
}
