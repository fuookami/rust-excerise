use rand::*;
use std::error::Error;
use std::fmt;

struct DepthOverError {
    depth: usize,
}

impl fmt::Debug for DepthOverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Depth over: {}", self.depth)
    }
}

impl fmt::Display for DepthOverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Depth over: {}", self.depth)
    }
}

impl Error for DepthOverError {}

fn foo(depth: usize) -> Result<(), DepthOverError> {
    let mut rng = thread_rng();
    match rng.next_u32() % 10 {
        0 => Result::Err(DepthOverError { depth: depth }),
        _ => foo(depth + 1),
    }
}

fn main() {
    println!("{}", foo(0).err().unwrap());
}
