pub struct LifeMatrix {
    cols: usize,
    rows: usize,
    size: usize,
    data: Vec<bool>,
}

impl LifeMatrix {
    pub fn new(cols: usize, rows: usize) -> LifeMatrix {
        let size = cols * rows;

        return LifeMatrix {
            cols,
            rows,
            size,
            data: vec![false; size],
        };
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.cols as i32 || y < 0 || y >= self.rows as i32 {
            return false;
        }

        let i = x + y * self.cols as i32;
        return self.data[i as usize];
    }

    pub fn set(&mut self, x: i32, y: i32, val: bool) {
        let i = x as usize + y as usize * self.cols;
        self.data[i] = val;
    }

    pub fn get_iter(&self) -> std::slice::Iter<'_, bool> {
        return self.data.iter();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty() {
        let mut matrix = LifeMatrix::new(8, 8);

        assert_eq!(matrix.get(0, 0), false);
        assert_eq!(matrix.get(7, 6), false);
    }

    #[test]
    fn can_set() {
        let mut matrix = LifeMatrix::new(8, 8);

        matrix.set(0, 0, true);
        matrix.set(6, 7, true);

        assert_eq!(matrix.get(0, 0), true);
        assert_eq!(matrix.get(6, 7), true);
        assert_eq!(matrix.get(7, 6), false);
    }

    #[test]
    fn get_iter() {
        let mut matrix = LifeMatrix::new(8, 8);

        matrix.set(1, 2, true);
        matrix.set(3, 4, true);
        matrix.set(5, 6, true);

        let res: i32 = matrix.get_iter().map(|x| {if *x { 1 } else { 0 }}).sum();

        assert_eq!(res, 3);
    }
}
