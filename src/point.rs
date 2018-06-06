#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn neighbors(&self) -> Vec<Option<Point>> {
        vec![self.left(), self.right(), self.up(), self.down()]
    }

    fn left(&self) -> Option<Point> {
        self.0.checked_sub(1).map(|x| Point(x, self.1))
    }

    fn right(&self) -> Option<Point> {
        self.0.checked_add(1).map(|x| Point(x, self.1))
    }

    fn up(&self) -> Option<Point> {
        self.1.checked_sub(1).map(|y| Point(self.0, y))
    }

    fn down(&self) -> Option<Point> {
        self.1.checked_add(1).map(|y| Point(self.0, y))
    }
}
