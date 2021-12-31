use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::{max, min},
    io::BufRead,
    iter,
};

use crate::helpers::matrix::{Matrix, Point};

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    solution(input, false)
}

pub fn part2(input: impl BufRead) -> anyhow::Result<String> {
    solution(input, true)
}

fn solution(input: impl BufRead, diagonals: bool) -> anyhow::Result<String> {
    let lines = parse_input(input)?.into_iter().collect::<Vec<_>>();
    let mut farthest_point: Point = (0, 0);
    for line in lines.iter() {
        farthest_point.0 = max(max(line.start.0, line.end.0), farthest_point.0);
        farthest_point.1 = max(max(line.start.1, line.end.1), farthest_point.1);
    }
    let mut matrix = Matrix::new(farthest_point.1 + 1, farthest_point.0 + 1, 0u8);
    for line in lines.iter() {
        if diagonals {
            for point in line.points_with_diagonals() {
                matrix[point] += 1;
            }
        } else {
            if line.is_horizontal() || line.is_vertical() {
                for point in line.points() {
                    matrix[point] += 1;
                }
            }
        }
    }
    let num_crossing_points = matrix.points().filter(|count| **count >= 2).count();
    Ok(format!("{}", num_crossing_points))
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        if self.is_vertical() {
            // Vertical
            let start_y = min(self.start.1, self.end.1);
            let end_y = max(self.start.1, self.end.1);
            Box::new(({ start_y..end_y + 1 }).map(|y| (self.start.0, y)))
        } else if self.is_horizontal() {
            // Horizontal
            let start_x = min(self.start.0, self.end.0);
            let end_x = max(self.start.0, self.end.0);
            Box::new(({ start_x..end_x + 1 }).map(|x| (x, self.start.1)))
        } else {
            unimplemented!()
        }
    }

    fn points_with_diagonals(&self) -> impl Iterator<Item = Point> + '_ {
        let horiz_distance = self.end.0 as isize - self.start.0 as isize;
        let vert_distance = self.end.1 as isize - self.start.1 as isize;
        assert!(
            horiz_distance.abs() == 0
                || vert_distance.abs() == 0
                || horiz_distance.abs() == vert_distance.abs()
        );
        let x_direction = horiz_distance.signum();
        let y_direction = vert_distance.signum();
        let mut point = self.start;
        iter::from_fn(move || {
            if point.0 != (self.end.0 as isize + x_direction) as usize
                || point.1 != (self.end.1 as isize + y_direction) as usize
            {
                let out = point;
                point.0 = (point.0 as isize + x_direction) as usize;
                point.1 = (point.1 as isize + y_direction) as usize;
                Some(out)
            } else {
                None
            }
        })
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
                    start: (captures[1].parse()?, captures[2].parse()?),
                    end: (captures[3].parse()?, captures[4].parse()?),
                })
            } else {
                Err(anyhow!("Line did not match regex"))
            }
        })
        .collect()
}
