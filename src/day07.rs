use anyhow::anyhow;
use std::io::BufRead;

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let input = parse_input(input)?;
    let minimum_distance = minimum_distance(&input, |a, b| (a - b).abs() as u32).unwrap();
    Ok(format!("{}", minimum_distance))
}

pub fn part2(input: impl BufRead) -> anyhow::Result<String> {
    let input = parse_input(input)?;
    let minimum_distance = minimum_distance(&input, |a, b| {
        let d = (a - b).abs() as u32;
        d * (d + 1) / 2
    })
    .unwrap();
    Ok(format!("{}", minimum_distance))
}

fn minimum_distance(positions: &[Disp], distance: impl Fn(Disp, Disp) -> u32) -> Option<u32> {
    if positions.is_empty() {
        return None;
    }
    let start = *positions.iter().min().unwrap();
    let end = *positions.iter().max().unwrap();
    let mut minimum_distance = u32::MAX;
    for alignment_position in start..end + 1 {
        let total_distance = positions
            .iter()
            .map(|pos| distance(*pos, alignment_position))
            .sum();
        if total_distance < minimum_distance {
            minimum_distance = total_distance;
        }
    }
    Some(minimum_distance)
}

type Disp = i32; // Displacement

fn parse_input(input: impl BufRead) -> anyhow::Result<Vec<Disp>> {
    input
        .lines()
        .next()
        .ok_or(anyhow!("Empty input"))??
        .split(',')
        .map(|n| n.parse().map_err(anyhow::Error::from))
        .collect()
}
