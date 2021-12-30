use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::{max, min},
    fmt::Display,
    io::BufRead,
    ops::{Index, IndexMut},
};

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let lines = parse_input(input)?
        .into_iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect::<Vec<_>>();
    let mut farthest_point = Point { x: 0, y: 0 };
    for line in lines.iter() {
        farthest_point.x = max(max(line.start.x, line.end.x), farthest_point.x);
        farthest_point.y = max(max(line.start.y, line.end.y), farthest_point.y);
    }
    eprintln!("{:?}", farthest_point);
    let mut matrix = Matrix::new(farthest_point.y + 1, farthest_point.x + 1, 0u8);
    for line in lines.iter() {
        for point in line.points() {
            matrix[point] += 1;
        }
    }
    let num_crossing_points = matrix.data.iter().filter(|count| **count >= 2).count();
    Ok(format!("{}", num_crossing_points))
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        if self.is_vertical() {
            // Vertical
            let start_y = min(self.start.y, self.end.y);
            let end_y = max(self.start.y, self.end.y);
            Box::new(({ start_y..end_y + 1 }).map(|y| Point { x: self.start.x, y }))
        } else if self.is_horizontal() {
            // Horizontal
            let start_x = min(self.start.x, self.end.x);
            let end_x = max(self.start.x, self.end.x);
            Box::new(({ start_x..end_x + 1}).map(|x| Point { x, y: self.start.y }))
        } else {
            unimplemented!()
        }
    }
}

struct Matrix<T> {
    cols: usize,
    data: Box<[T]>,
}

impl<T: Copy> Matrix<T> {
    fn new(rows: usize, cols: usize, fill: T) -> Self {
        let data = [fill].repeat(rows * cols).into_boxed_slice();
        Matrix { cols, data }
    }
}

impl<T> Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.y * self.cols + index.x]
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[index.y * self.cols + index.x]
    }
}

impl Display for Matrix<u8> {
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

fn parse_input(input: impl BufRead) -> anyhow::Result<Vec<Line>> {
    input
        .lines()
        .map(|line| {
            lazy_static! {
                static ref LINE_FMT: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
            }
            if let Some(captures) = LINE_FMT.captures(&line?) {
                Ok(Line {
                    start: Point {
                        x: captures[1].parse()?,
                        y: captures[2].parse()?,
                    },
                    end: Point {
                        x: captures[3].parse()?,
                        y: captures[4].parse()?,
                    },
                })
            } else {
                Err(anyhow!("Line did not match regex"))
            }
        })
        .collect()
}
