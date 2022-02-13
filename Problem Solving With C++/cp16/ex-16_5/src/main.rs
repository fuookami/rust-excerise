use std::error::Error;
use std::fmt;
use std::mem::swap;

struct StackOverflowError {}

impl fmt::Debug for StackOverflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack overflow!")
    }
}

impl fmt::Display for StackOverflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack overflow!")
    }
}

impl Error for StackOverflowError {}

struct StackEmptyError {}

impl fmt::Debug for StackEmptyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack overflow!")
    }
}

impl fmt::Display for StackEmptyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack overflow!")
    }
}

impl Error for StackEmptyError {}

struct Stack<T> {
    array: Vec<Option<T>>,
    top: usize,
}

impl<T> Stack<T> {
    fn push<'a>(&mut self, ele: T) -> Result<(), StackOverflowError> {
        if self.top <= self.array.len() {
            self.array[self.top - 1] = Option::Some(ele);
            self.top += 1;
            Result::Ok(())
        } else {
            Result::Err(StackOverflowError {})
        }
    }

    fn pop(&mut self, ele: T) -> Result<T, StackEmptyError> {
        if self.top > 0 {
            let mut ret = Option::None;
            swap(&mut ret, &mut self.array[self.top - 1]);
            self.top -= 1;
            Result::Ok(ret.unwrap())
        } else {
            Result::Err(StackEmptyError {})
        }
    }
}

fn main() {
    println!("Hello, world!");
}
