use std::io::BufRead;

use crate::helpers::matrix::Matrix;

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let mut matrix = parse_input(input)?;
    let mut total_flashed = 0;
    for _ in 0..100 {
        total_flashed += iterate_flash_step(&mut matrix);
    }
    Ok(format!("{}", total_flashed))
}

fn parse_input(mut input: impl BufRead) -> anyhow::Result<Matrix<u8>> {
    let mut input_string = String::new();
    input.read_to_string(&mut input_string)?;
    Matrix::parse_from_table(&input_string)
}

fn iterate_flash_step(matrix: &mut Matrix<u8>) -> usize {
    eprintln!("{:?}", OctopusMatrix(&matrix));
    eprintln!("begin step");
    // First
    for value in matrix.value_mut() {
        *value = value.saturating_add(1);
    }

    // Then
    let mut flashed_coords_all = Vec::new();
    let mut has_flashed = true;
    while has_flashed {
        eprintln!("begin round");
        let mut flashed_coords_this_round = Vec::new();
        has_flashed = false;
        for (point, value) in matrix.enumerate() {
            if !flashed_coords_all.contains(&point) && value > 9 {
                eprintln!("flash at {:?}", point);
                has_flashed = true;
                flashed_coords_this_round.push(point);
            }
        }

        for coord in &flashed_coords_this_round {
            matrix[*coord] = matrix[*coord].saturating_add(1);
            for surrounding_coord in coord.surrounding_points(true) {
                if let Some(val) = matrix.get_mut(surrounding_coord) {
                    *val = val.saturating_add(1);
                }
            }
        }

        flashed_coords_all.extend_from_slice(&flashed_coords_this_round);
    }

    // Finally
    for coord in &flashed_coords_all {
        matrix[*coord] = 0;
    }

    flashed_coords_all.len()
}

struct OctopusMatrix<'m>(pub &'m Matrix<u8>);

impl<'m> std::ops::Deref for OctopusMatrix<'m> {
    type Target = Matrix<u8>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'m> std::fmt::Debug for OctopusMatrix<'m> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            for datum in row.iter() {
                if *datum == 0 {
                    write!(f, "\x1b[97m{:}\x1b[0m", datum)?;
                } else {
                    write!(f, "{:}", datum)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
