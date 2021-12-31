use std::io::BufRead;

use crate::helpers::matrix::Matrix;

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let input = parse_input(input)?;
    eprintln!("{:?}", input);

    // Find low points
    let mut low_points = Vec::new();
    for ((x, y), value) in input.enumerate() {
        let mut value_is_lowest = true;
        if x > 0 {
            value_is_lowest &= input[(x - 1, y)] > value;
        }
        if x < input.cols() - 1 {
            value_is_lowest &= input[(x + 1, y)] > value;
        }
        if y > 0 {
            value_is_lowest &= input[(x, y - 1)] > value;
        }
        if y < input.rows() - 1 {
            value_is_lowest &= input[(x, y + 1)] > value;
        }
        if value_is_lowest {
            low_points.push((x, y));
        }
    }

    // Sum low point "risk levels" (value + 1)
    let sum = low_points
        .into_iter()
        .map(|point| input[point] + 1)
        .fold(0u32, |t, v| t + v as u32);
    Ok(format!("{}", sum))
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
