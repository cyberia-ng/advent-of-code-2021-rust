use std::io::BufRead;

use crate::helpers::matrix::{Matrix, Point};

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let input = parse_input(input)?;
    let low_points = find_low_points(&input);

    // Sum low point "risk levels" (value + 1)
    let sum = low_points
        .into_iter()
        .map(|point| input[point] + 1)
        .fold(0u32, |t, v| t + v as u32);
    Ok(format!("{}", sum))
}

pub fn part2(input: impl BufRead) -> anyhow::Result<String> {
    let input = parse_input(input)?;
    let low_points = find_low_points(&input);
    let basins: Vec<Vec<Point>> = low_points
        .iter()
        .map(|low_point| find_basin(low_point, &input))
        .collect();
    let mut basin_sizes: Vec<usize> = basins.iter().map(|b| b.len()).collect();
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let answer: usize = basin_sizes[0..3].iter().product();
    Ok(format!("{}", answer))
}

type Input = Matrix<u8>;

fn parse_input(input: impl BufRead) -> anyhow::Result<Input> {
    let lines = input
        .lines()
        .map(|line| line.map_err(anyhow::Error::from))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let rows = lines.len();
    let cols = lines[0].len();
    let mut input = Matrix::new(rows, cols, 0u8);
    for (row_idx, line) in lines.iter().enumerate() {
        for (col_idx, char_) in line.chars().enumerate() {
            input[(col_idx, row_idx)] = String::from(char_).parse()?;
        }
    }
    Ok(input)
}

fn surrounding_points(point: &Point, boundary: &Point) -> impl Iterator<Item = Point> {
    const SURROUNDING_RELATIVE: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let point = *point;
    let boundary = *boundary;
    SURROUNDING_RELATIVE
        .iter()
        .map(move |rel| (point.0 as isize + rel.0, point.1 as isize + rel.1))
        .filter(move |out| {
            out.0 >= 0 && out.0 < boundary.0 as isize && out.1 >= 0 && out.1 < boundary.1 as isize
        })
        .map(|out| (out.0 as usize, out.1 as usize))
}

fn find_low_points(matrix: &Matrix<u8>) -> Vec<Point> {
    let mut low_points = Vec::new();
    for ((x, y), value) in matrix.enumerate() {
        let mut value_is_lowest = true;
        for point in surrounding_points(&(x, y), &(matrix.cols(), matrix.rows())) {
            value_is_lowest &= matrix[point] > value;
        }
        if value_is_lowest {
            low_points.push((x, y));
        }
    }
    low_points
}

const BASIN_LIMIT: u8 = 9;

fn find_basin(start: &Point, matrix: &Matrix<u8>) -> Vec<Point> {
    let mut to_visit = vec![*start];
    let mut visited = Vec::new();
    let mut basin = Vec::new();
    let boundary = (matrix.cols(), matrix.rows());
    while let Some(point) = to_visit.pop() {
        if visited.contains(&point) {
            continue;
        }
        visited.push(point);
        if matrix[point] < BASIN_LIMIT {
            basin.push(point);
            to_visit.extend(surrounding_points(&point, &boundary));
        }
    }
    basin
}
