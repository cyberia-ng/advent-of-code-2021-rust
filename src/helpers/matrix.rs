use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

pub type Point = (usize, usize);

pub struct Matrix<T> {
    cols: usize,
    data: Box<[T]>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(rows: usize, cols: usize, fill: T) -> Self {
        let data = [fill].repeat(rows * cols).into_boxed_slice();
        Matrix { cols, data }
    }

    pub fn rows(&self) -> usize {
        self.data.len() / self.cols
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn points(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Point, T)> + '_ {
        ({ 0..self.rows() })
            .flat_map(move |y| ({ 0..self.cols }).map(move |x| ((x, y), self[(x, y)])))
    }
}

impl<T> Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.1 * self.cols + index.0]
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[index.1 * self.cols + index.0]
    }
}

impl Debug for Matrix<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self.data.len() / self.cols;
        for row in self.data.chunks(rows) {
            for datum in row.iter() {
                write!(f, "{:>3} ", datum)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
