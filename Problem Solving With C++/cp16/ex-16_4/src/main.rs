use std::error::Error;
use std::fmt;
use std::ops::Index;

struct ArrayOutOfRangeError {
    index: usize,
}

impl fmt::Debug for ArrayOutOfRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Out Of Range of index {}", self.index)
    }
}

impl fmt::Display for ArrayOutOfRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Out Of Range of index {}", self.index)
    }
}

impl Error for ArrayOutOfRangeError {}

struct CheckedArray<T> {
    array: Vec<T>,
}

impl<T> CheckedArray<T> {
    fn get<'a>(&'a self, index: usize) -> Result<&'a T, ArrayOutOfRangeError> {
        if index < self.array.len() {
            Result::Ok(&self.array[index])
        } else {
            Result::Err(ArrayOutOfRangeError { index })
        }
    }
}

fn main() {
    println!("Hello, world!");
}
