use std::alloc::*;
use std::cmp;
use std::fmt;
use std::mem;
use std::ops;

struct StringVar {
    ptr: *mut char,
    len: usize,
    _size: usize,
}

impl ops::Index<usize> for StringVar {
    type Output = char;

    fn index(&self, pos: usize) -> &char {
        unsafe { &*self.ptr.wrapping_offset(pos as isize) }
    }
}

impl ops::IndexMut<usize> for StringVar {
    fn index_mut(&mut self, pos: usize) -> &mut char {
        unsafe { &mut *self.ptr.wrapping_offset(pos as isize) }
    }
}

impl StringVar {
    const ALIGN: usize = mem::align_of::<char>();
    const ELEM_SIZE: usize = mem::size_of::<char>();

    fn new() -> Self {
        Self::new_from_length(100)
    }

    fn new_from_length(len: usize) -> Self {
        Self {
            ptr: Self::_alloc(len),
            len: 0,
            _size: len,
        }
    }

    fn new_from_str(s: &str) -> Self {
        let mut ret = Self::new_from_length(s.len());
        let mut i = 0;
        for ch in s.chars() {
            ret[i] = ch;
            i += 1;
        }
        ret.len = i;
        ret
    }

    fn length(&self) -> usize {
        self.len
    }

    fn clone_piece(&self, pos: usize, len: usize) -> Self {
        let mut ret = Self::new_from_length(len);
        for i in 0..len {
            ret[i] = self[pos + i];
        }
        ret
    }

    fn _alloc(len: usize) -> *mut char {
        unsafe {
            let allocator = System {};
            allocator.alloc(Layout::from_size_align(Self::ELEM_SIZE * len, Self::ALIGN).unwrap())
                as *mut char
        }
    }
}

impl Drop for StringVar {
    fn drop(&mut self) {
        unsafe {
            let allocator = System {};
            allocator.dealloc(
                self.ptr as *mut u8,
                Layout::from_size_align(Self::ELEM_SIZE * self._size, Self::ALIGN).unwrap(),
            );
        }
    }
}

impl Clone for StringVar {
    fn clone(&self) -> Self {
        let mut ret = Self {
            ptr: Self::_alloc(self._size),
            len: self.len,
            _size: self._size,
        };
        for i in 0..self.len {
            ret[i] = self[i];
        }
        ret
    }
}

impl ops::Add<&StringVar> for StringVar {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        let mut ret = Self::new_from_length(self.length() + rhs.length());
        for i in 0..self.length() {
            ret[i] = self[i];
        }
        for i in 0..rhs.length() {
            ret[i + self.length()] = rhs[i];
        }
        ret
    }
}

impl cmp::PartialEq for StringVar {
    fn eq(&self, other: &Self) -> bool {
        if self.length() != other.length() {
            return false;
        }
        for i in 0..self.length() {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for StringVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.length() {
            let result = write!(f, "{}", self[i]);
            if result.is_err() {
                return fmt::Result::Err(result.err().unwrap());
            }
        }
        return fmt::Result::Ok(());
    }
}

struct Text {
    strs: Vec<StringVar>,
}

impl Text {
    fn new(line: &str) -> Text {
        let mut strs = Vec::new();
        for s in line.split_ascii_whitespace() {
            strs.push(StringVar::new_from_str(s));
        }
        Text { strs: strs }
    }
}

fn main() {
    println!("Hello, world!");
}
