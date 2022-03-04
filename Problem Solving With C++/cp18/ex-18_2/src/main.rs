struct PrimeGenrator {
    _flag: Vec<bool>,
}

impl PrimeGenrator {
    fn new() -> Self {
        Self {
            _flag: vec![false, false, true],
        }
    }

    fn is(&mut self, value: u64) -> bool {
        self.gen(value);
        self._flag[value as usize]
    }

    fn gen(&mut self, value: u64) {
        let upper_bound = (value as f64).sqrt().ceil() as u64;
        self._flag.resize(value as usize + 1, true);
        for i in 2..=upper_bound {
            if self._flag[i as usize] {
                for j in 2..value {
                    let k = i * j;
                    if k > value {
                        break;
                    }
                    self._flag[k as usize] = false;
                }
            }
        }
    }
}

fn main() {
    let mut gen = PrimeGenrator::new();
    assert!(gen.is(5));
    assert!(!gen.is(10));
    assert!(!gen.is(9));
}
