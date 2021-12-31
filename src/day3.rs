use anyhow::anyhow;
use std::{io::BufRead, mem::size_of};

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let (ints, max_bit_depth) = parse_input(input)?;
    let mut bitwise_average = 0;
    for bit_idx in 0..INT_BIT_DEPTH {
        bitwise_average |= bit_average(&ints, bit_idx);
    }
    let anti_average_mask = (1 << max_bit_depth) - 1;
    let anti_average = bitwise_average ^ anti_average_mask;
    Ok(format!("{}", bitwise_average * anti_average))
}

pub fn part2(input: impl BufRead) -> anyhow::Result<String> {
    let (ints, max_bit_depth) = parse_input(input)?;
    let thing1 = bitwise_similarity_filter(&ints, max_bit_depth, false)?;
    let thing2 = bitwise_similarity_filter(&ints, max_bit_depth, true)?;
    Ok(format!("{}", thing1 * thing2))
}

type IntType = u32;
const INT_BIT_DEPTH: usize = size_of::<IntType>() * 8;

fn parse_input(input: impl BufRead) -> anyhow::Result<(Vec<IntType>, usize)> {
    let mut max_line_length = 0;
    let ints = input
        .lines()
        .map(|line| {
            let line = line?;
            let len = line.len();
            assert!(len <= INT_BIT_DEPTH);
            if max_line_length < len {
                max_line_length = len
            }
            let mut out: IntType = 0;
            for char_ in line.chars() {
                out <<= 1;
                match char_ {
                    '1' => out |= 1,
                    '0' => {}
                    _ => return Err(anyhow!("Input line was not binary")),
                };
            }
            Ok(out)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok((ints, max_line_length))
}

fn bit_average(ints: &[IntType], bit_idx: usize) -> IntType {
    let mut count = 0usize;
    let mask: IntType = 1 << (INT_BIT_DEPTH - bit_idx - 1);
    for int in ints {
        if int & mask > 0 {
            count += 1;
        }
    }
    if 2 * count >= ints.len() {
        mask
    } else {
        0
    }
}

fn bitwise_similarity_filter(
    ints: &[IntType],
    max_bit_depth: usize,
    anti: bool,
) -> anyhow::Result<IntType> {
    let mut out: Vec<IntType> = ints.to_vec();
    for bit_idx in (INT_BIT_DEPTH - max_bit_depth)..INT_BIT_DEPTH {
        if out.len() == 1 {
            break;
        }
        let bit_average = bit_average(&out, bit_idx);
        let mask: IntType = 1 << (INT_BIT_DEPTH - bit_idx - 1);
        out = out
            .into_iter()
            .filter(|int| anti ^ ((int & mask) ^ bit_average == 0))
            .collect::<Vec<_>>();
    }
    Ok(*out.get(0).ok_or(anyhow!("Empty"))?)
}
