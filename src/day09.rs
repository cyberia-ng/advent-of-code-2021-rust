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

fn parse_input(mut input: impl BufRead) -> anyhow::Result<Input> {
    let mut input_string = String::new();
    input.read_to_string(&mut input_string)?;
    Ok(Matrix::parse_from_table(&input_string)?)
}

fn find_low_points(matrix: &Matrix<u8>) -> Vec<Point> {
    let mut low_points = Vec::new();
    for (point, value) in matrix.enumerate() {
        let mut value_is_lowest = true;
        for surrounding_point in point.surrounding_points(false) {
            if let Some(value_of_surrounding) = matrix.get(surrounding_point) {
                value_is_lowest &= *value_of_surrounding > value;
            }
        }
        if value_is_lowest {
            low_points.push(point);
        }
    }
    low_points
}

const BASIN_LIMIT: u8 = 9;

fn find_basin(start: &Point, matrix: &Matrix<u8>) -> Vec<Point> {
    let mut to_visit = vec![*start];
    let mut visited = Vec::new();
    let mut basin = Vec::new();
    while let Some(point) = to_visit.pop() {
        if visited.contains(&point) {
            continue;
        }
        visited.push(point);
        if let Some(value) = matrix.get(point) {
            if *value < BASIN_LIMIT {
                basin.push(point);
                to_visit.extend(point.surrounding_points(false));
            }
        }
    }
    basin
}
