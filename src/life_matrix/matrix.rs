#[derive(Clone)]
pub struct Matrix {
    m: u64,
}

// TODO iterator
// TODO hashing

impl Matrix {
    pub fn new(initial: u64) -> Matrix {
        Matrix { m: initial }
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        assert!(x < 8);
        assert!(y < 8);
        let bitmask = 1 << (7 - x) << (7 - y) * 8;
        return self.m & bitmask > 0;
    }

    pub fn set(&mut self, x: i32, y: i32, val: bool) {
        assert!(x < 8);
        assert!(y < 8);

        let bitmask = 1 << (7 - x) << (7 - y) * 8;
        if val {
            self.m = self.m | bitmask;
        } else {
            self.m = self.m & !bitmask;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_can_set_and_get() {
        let mut matrix = Matrix::new(0b100 << 8 | 0b11);

        // get uninitialized bits
        assert_eq!(matrix.get(0, 0), false);
        assert_eq!(matrix.get(2, 3), false);

        // get initialized bits
        assert_eq!(matrix.get(5, 6), true);
        assert_eq!(matrix.get(6, 7), true);
        assert_eq!(matrix.get(7, 7), true);

        // modify initialized bits
        matrix.set(0, 0, false);
        matrix.set(2, 3, true);
        matrix.set(5, 6, false);

        // modify uninitialized bits
        matrix.set(6, 5, true);

        // get overwritten bits
        assert_eq!(matrix.get(0, 0), false);
        assert_eq!(matrix.get(2, 3), true);
        assert_eq!(matrix.get(5, 6), false);
        assert_eq!(matrix.get(6, 5), true);
    }
}
