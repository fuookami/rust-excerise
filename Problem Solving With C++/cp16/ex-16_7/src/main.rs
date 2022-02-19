use std::error::Error;
use std::fmt;

struct ArrayOutOfRangeError {
    row: Option<usize>,
    col: Option<usize>,
}

impl fmt::Debug for ArrayOutOfRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        if let (Option::Some(row), Option::Some(col)) = (self.row, self.col) {
            write!(f, "Out of range row and col: ({}, {})", row, col)
        } else if let Option::Some(row) = self.row {
            write!(f, "Out of range row: {}", row)
        } else if let Option::Some(col) = self.col {
            write!(f, "Out of range col: {}", col)
        } else {
            panic!("Imposible branch!!!");
        }
    }
}

impl fmt::Display for ArrayOutOfRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        (self as &dyn fmt::Debug).fmt(f)
    }
}

impl Error for ArrayOutOfRangeError {}

struct MatrixSize {
    rows: usize,
    cols: usize,
}

struct Matrix {
    vals: Vec<i32>,
    size: MatrixSize,
}

impl MatrixSize {
    fn index(&self, row: usize, col: usize) -> Result<usize, ArrayOutOfRangeError> {
        if row >= self.rows && col >= self.cols {
            Result::Err(ArrayOutOfRangeError {
                row: Option::Some(row),
                col: Option::Some(col),
            })
        } else if row >= self.rows {
            Result::Err(ArrayOutOfRangeError {
                row: Option::Some(row),
                col: Option::None,
            })
        } else if col >= self.cols {
            Result::Err(ArrayOutOfRangeError {
                row: Option::None,
                col: Option::Some(col),
            })
        } else {
            Result::Ok(row * self.cols + col)
        }
    }
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Matrix {
        let mut vals = Vec::new();
        vals.resize(rows * cols, 0);
        Matrix {
            vals: vals,
            size: MatrixSize {
                rows: rows,
                cols: cols,
            },
        }
    }

    fn set(&mut self, row: usize, col: usize, val: i32) -> Result<(), ArrayOutOfRangeError> {
        match self.size.index(row, col) {
            Result::Ok(i) => {
                self.vals[i] = val;
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    fn get(&self, row: usize, col: usize) -> Result<i32, ArrayOutOfRangeError> {
        match self.size.index(row, col) {
            Result::Ok(i) => Result::Ok(self.vals[i]),
            Result::Err(err) => Result::Err(err),
        }
    }
}

fn main() {
    let mut mat = Matrix::new(2, 3);
    assert_eq!(mat.get(0, 0).unwrap(), 0);
    assert_eq!(mat.get(0, 1).unwrap(), 0);

    mat.set(0, 0, 1);
    assert_eq!(mat.get(0, 0).unwrap(), 1);
    assert_eq!(mat.get(0, 1).unwrap(), 0);
}
