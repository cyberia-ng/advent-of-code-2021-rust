use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn surrounding_points(&self, diagonals: bool) -> impl Iterator<Item = Point> {
        let mut points = Vec::with_capacity(8);
        points.extend_from_slice(&[(-1, 0), (1, 0), (0, -1), (0, 1)]);
        if diagonals {
            points.extend_from_slice(&[(-1, -1), (1, -1), (1, 1), (-1, 1)]);
        }
        let point = *self;

        points
            .into_iter()
            .map(move |rel| (point.0 as isize + rel.0, point.1 as isize + rel.1))
            .filter(|out| out.0 >= 0 && out.1 >= 0)
            .map(|out| Point(out.0 as usize, out.1 as usize))
    }
}

pub struct Matrix<T> {
    cols: usize,
    data: Box<[T]>,
}

impl<T> Matrix<T> {
    pub fn num_rows(&self) -> usize {
        self.data.len() / self.cols
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> + '_ {
        self.data.chunks(self.num_cols())
    }

    pub fn num_cols(&self) -> usize {
        self.cols
    }

    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
    }

    pub fn value_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.data.iter_mut()
    }

    pub fn get(&self, point_index: Point) -> Option<&T> {
        if point_index.0 >= self.num_cols() || point_index.1 >= self.num_rows() {
            return None;
        }
        let data_index = point_index.1 * self.cols + point_index.0;
        self.data.get(data_index)
    }

    pub fn get_mut(&mut self, index: Point) -> Option<&mut T> {
        if index.0 >= self.num_cols() || index.1 >= self.num_rows() {
            return None;
        }
        let data_index = index.1 * self.cols + index.0;
        self.data.get_mut(data_index)
    }
}

impl<T: Copy> Matrix<T> {
    pub fn new(rows: usize, cols: usize, fill: T) -> Self {
        let data = [fill].repeat(rows * cols).into_boxed_slice();
        Matrix { cols, data }
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Point, T)> + '_ {
        ({ 0..self.num_rows() })
            .flat_map(move |y| ({ 0..self.cols }).map(move |x| (Point(x, y), self[Point(x, y)])))
    }
}

impl<T> Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Debug for Matrix<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            for datum in row.iter() {
                write!(f, "{:>3} ", datum)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Matrix<u8> {
    pub fn parse_from_table(table: &str) -> anyhow::Result<Self> {
        let lines: Vec<&str> = table.lines().collect();
        let rows = lines.len();
        let cols = lines[0].len();
        let mut out = Matrix::new(rows, cols, 0u8);
        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, char_) in line.chars().enumerate() {
                out[Point(col_idx, row_idx)] = String::from(char_).parse()?;
            }
        }
        Ok(out)
    }
}
