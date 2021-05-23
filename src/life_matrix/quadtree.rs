use super::matrix::Matrix;
use sdl2::rect::Point;
use std::sync::Arc;
use std::sync::Weak;

pub struct QuadTreeMatrix {
    root: Option<Weak<QuadTreeMatrix>>,
    initialized: bool,
    pos: Point,
    width: i32,
    height: i32,
    depth: i32, // TODO: this may completely replace widht/height
    top_left: Edge,
    top_right: Edge,
    bottom_left: Edge,
    bottom_right: Edge,
}

// TODO: can this be replaced with a trait?
#[derive(Clone)]
enum Edge {
    Child(Arc<QuadTreeMatrix>),
    Leaf(Matrix),
    Empty,
}

// TODO: start uninitialized, then instantiate when set
impl QuadTreeMatrix {
    pub fn new() -> QuadTreeMatrix {
        return QuadTreeMatrix {
            root: None, // I'm the root
            initialized: true,
            pos: Point::new(0, 0),
            width: 16,
            height: 16,
            depth: 1,
            top_left: Edge::Leaf(Matrix::new(0)),
            top_right: Edge::Leaf(Matrix::new(0)),
            bottom_left: Edge::Leaf(Matrix::new(0)),
            bottom_right: Edge::Leaf(Matrix::new(0)),
        };
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }

        let half_width = self.width / 2;
        let half_height = self.height / 2;

        if x < half_width {
            if y < half_height {
                return self.top_left.get(x, y);
            } else {
                return self.bottom_left.get(x, y - half_height);
            }
        } else {
            if y < half_height {
                return self.top_right.get(x - half_width, y);
            } else {
                return self.bottom_right.get(x - half_width, y - half_height);
            }
        }
    }

    pub fn set(&mut self, x: i32, y: i32, val: bool) {
        println!("quadtree set {} {}", x, y);

        if x < self.pos.x || y < self.pos.y {
            // TODO
            println!("Grow as BR");
        } else if x > self.width + self.pos.x || y > self.height + self.pos.y {
            println!("Grow as TL");
            let top_left = self.create_standin();
            self.width = self.width * 2;
            self.height = self.height * 2;
            self.top_left = Edge::Child(Arc::new(top_left));

            self.set(x, y, val);
        } else {
            let half_width = self.width / 2;
            let half_height = self.height / 2;

            if x < half_width {
                if y < half_height {
                    return self.top_left.set(x, y, val);
                } else {
                    return self.bottom_left.set(x, y - half_height, val);
                }
            } else {
                if y < half_height {
                    return self.top_right.set(x - half_width, y, val);
                } else {
                    return self.bottom_right.set(x - half_width, y - half_height, val);
                }
            }
        }
    }

    fn create_standin(&mut self) -> QuadTreeMatrix {
        QuadTreeMatrix {
            root: None,
            initialized: self.initialized,
            pos: self.pos.clone(),
            width: self.width,
            height: self.height,
            depth: self.depth,
            top_left: std::mem::replace(&mut self.top_left, Edge::Empty),
            top_right: std::mem::replace(&mut self.top_right, Edge::Empty),
            bottom_left: std::mem::replace(&mut self.bottom_left, Edge::Empty),
            bottom_right: std::mem::replace(&mut self.bottom_right, Edge::Empty),
        }
    }
}

impl Edge {
    fn get(&self, x: i32, y: i32) -> bool {
        match self {
            Edge::Child(tree) => tree.get(x, y),
            Edge::Leaf(matrix) => matrix.get(x, y),
            Edge::Empty => false,
        }
    }

    fn set(&mut self, x: i32, y: i32, val: bool) {
        match self {
            Edge::Child(ref mut tree) => {
                //let tree = Arc::get_mut(&mut tree).unwrap();
                Arc::get_mut(tree).unwrap().set(x, y, val);
            }
            Edge::Leaf(matrix) => matrix.set(x, y, val),
            Edge::Empty => {
                // TODO: handle creating a new tree/matrix here
                panic!("Not implemented");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadtree_can_set_and_get() {
        let mut matrix = QuadTreeMatrix::new();

        // get uninitialized bits
        assert_eq!(matrix.get(0, 0), false); // top_left
        assert_eq!(matrix.get(13, 7), false); // top_right

        matrix.set(0, 0, false); // top_left
        matrix.set(13, 7, true); // top_right
        matrix.set(7, 13, true); // bottom_left
        matrix.set(9, 8, false); // bottom_right

        // get initialized bits
        assert_eq!(matrix.get(0, 0), false); // top_left
        assert_eq!(matrix.get(13, 7), true); // top_right
        assert_eq!(matrix.get(7, 13), true); // bottom_left
        assert_eq!(matrix.get(9, 8), false); // bottom_right
    }

    #[test]
    fn quadtree_can_get_outside_of_tree() {
        let mut matrix = QuadTreeMatrix::new();

        assert_eq!(matrix.get(23, 10), false); // outside top_right
        assert_eq!(matrix.get(10, 23), false); // outside bottom_left
        assert_eq!(matrix.get(23, 23), false); // outside bottom_right
    }

    #[test]
    fn quadtree_can_grow() {
        let mut matrix = QuadTreeMatrix::new();

        matrix.set(9, 23, false); // outside bottom_left
        matrix.set(23, 23, true); // outside bottom_right
        matrix.set(23, 10, true); // outside top_right

        assert_eq!(matrix.get(9, 23), false); // bottom_left
        assert_eq!(matrix.get(23, 23), true); // bottom_right
        assert_eq!(matrix.get(23, 10), true); // top_right

        // TODO test growing to the top left
    }
}
