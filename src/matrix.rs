pub struct Matrix {
    cols: usize,
    rows: usize,
    size: usize,
    data: Vec<bool>,
}

impl Matrix {
    pub fn new(cols: usize, rows: usize) -> Matrix {
        let size = cols * rows;

        return Matrix {
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
