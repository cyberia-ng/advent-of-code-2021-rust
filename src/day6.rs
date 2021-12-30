use anyhow::anyhow;
use std::io::BufRead;

const DAYS_PART1: u8 = 80;
const DAYS_PART2: u16 = 256;

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let mut states = parse_input(input)?;
    for _ in 0..DAYS_PART1 {
        iterate(&mut states);
    }
    Ok(format!("{}", states.len()))
}

pub fn part2(input: impl BufRead) -> anyhow::Result<String> {
    let mut states = parse_input(input)?;
    for _ in 0..DAYS_PART2 {
        eprintln!("{}", states.len());
        iterate(&mut states);
    }
    Ok(format!("{}", states.len()))
}

fn parse_input(input: impl BufRead) -> anyhow::Result<Vec<u8>> {
    Ok(input
        .lines()
        .next()
        .ok_or(anyhow!("Empty input"))??
        .split(',')
        .map(|n| n.parse())
        .collect::<Result<Vec<_>, _>>()?)
}

fn iterate(states: &mut Vec<u8>) {
    let mut to_spawn = 0usize;
    for counter in states.iter_mut() {
        if *counter == 0 {
            *counter = 6;
            to_spawn += 1;
        } else {
            *counter -= 1;
        }
    }
    states.reserve(to_spawn);
    for _ in 0..to_spawn {
        states.push(8u8)
    }
}
