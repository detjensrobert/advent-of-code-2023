use std::ops::Index;

/// Grid of characters
struct Grid<T> {
  lines: Vec<Vec<T>>
}
type Point = (usize, usize);

/// get cell from coord pair
impl<T> Index<Point> for Grid<T> {
  type Output = T;

  fn index(&self, coord: Point) -> &T {
    &self.lines[coord.0][coord.1]
  }
}


impl<T> Grid<T> {
  fn neigh_up(&self, pos: Point) -> Option<T> {
    self.get(pos)
  }
}
