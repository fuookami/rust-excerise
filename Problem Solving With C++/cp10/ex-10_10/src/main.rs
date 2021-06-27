use bit_set::BitSet;
use std::ops::Index;

struct ZipCode {
    postnet: BitSet,
}

impl ZipCode {
    const WEIGHTS: [u8; 5] = [7, 4, 2, 1, 0];

    fn new_from_int_zip(mut zip: u32) -> Self {
        let mut v = BitSet::new();
        for i in (0..5).rev() {
            Self::from_int_zip(&mut v, (zip % 10) as u8, i * 5);
            zip /= 10;
        }
        v.insert(26);
        Self { postnet: v }
    }

    fn new_from_string_code(zip: &str) -> Self {
        let mut v = BitSet::new();
        let mut i = 0;
        for ch in zip.chars() {
            match ch {
                '1' => {
                    v.insert(i);
                }
                '0' => (),
                _ => panic!("Invalid string code!"),
            };
            i += 1;
        }
        Self { postnet: v }
    }

    fn int_zip(&self) -> u32 {
        let v = self.postnet.clone().into_bit_vec();
        let mut ret = 0;
        assert_eq!(v.len(), 26);
        for i in 0..5 {
            ret *= 10;
            ret += Self::cal_int_zip(&v, i * 5) as u32;
        }
        ret
    }

    fn string_code(&self) -> String {
        let v = self.postnet.clone().into_bit_vec();
        let mut ret = String::new();
        for bit in v {
            ret.push(if bit { '1' } else { '0' });
        }
        ret
    }

    fn from_int_zip(v: &mut BitSet, mut code: u8, i: usize) {
        if code == 0 {
            code = 11;
        }
        for j in 0..5 {
            if code > Self::WEIGHTS[j] {
                code -= Self::WEIGHTS[j];
                v.insert(i + j);
            }
        }
    }

    fn cal_int_zip<V: Index<usize, Output = bool>>(v: &V, i: usize) -> u8 {
        let mut ret = 0;
        for j in 0..5 {
            if v[i + j] {
                ret += Self::WEIGHTS[j];
            }
        }
        if ret == 11 {
            ret = 0
        }
        ret
    }
}

fn main() {
    assert_eq!(
        ZipCode::new_from_string_code("10100101000101011000010011").int_zip(),
        99504
    );
}
