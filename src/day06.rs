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
    use part2::*;
    let mut states = States::new_from(parse_input(input)?);
    for _ in 0..DAYS_PART2 {
        states.iterate();
    }
    Ok(format!("{}", states.total()))
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

mod part2 {
    type Count = u64;

    const MAX_CYCLE_LENGTH: usize = 9;
    pub struct States {
        counts: [Count; MAX_CYCLE_LENGTH],
    }

    const INIT: usize = 8;
    const AFTER_ZERO: usize = 6;

    impl States {
        pub fn empty() -> Self {
            Self {
                counts: [0; MAX_CYCLE_LENGTH],
            }
        }

        pub fn new_from(individuals: Vec<u8>) -> Self {
            let mut states = Self::empty();
            for individual in individuals {
                assert!((individual as usize) < MAX_CYCLE_LENGTH);
                states.counts[individual as usize] += 1;
            }
            states
        }

        pub fn iterate(&mut self) {
            let old_zero = self.counts[0];
            self.counts[0] = 0;
            self.counts.rotate_left(1);
            self.counts[AFTER_ZERO] += old_zero;
            self.counts[INIT] += old_zero;
        }

        pub fn total(&self) -> Count {
            self.counts.iter().sum()
        }
    }
}
