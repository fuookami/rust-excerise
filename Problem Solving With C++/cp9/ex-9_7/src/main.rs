struct MatrixSize {
    rows: usize,
    cols: usize
}

struct Matrix {
    vals: Vec<i32>,
    size: MatrixSize,
}

impl MatrixSize {
    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(row * self.cols + col)
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
                cols: cols
            }
        }
    }

    fn set(&mut self, row: usize, col: usize, val: i32) {
        match self.size.index(row, col) {
            Some(i) => self.vals[i] = val,
            None => {}
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<i32> {
        match self.size.index(row, col) {
            Some(i) => Some(self.vals[i]),
            None => None
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
