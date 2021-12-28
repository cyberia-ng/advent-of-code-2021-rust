use anyhow::anyhow;
use std::io::BufRead;

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let mut lines = input.lines().peekable();
    let reading_size = match lines.peek() {
        None => return Err(anyhow!("Empty input")),
        Some(Err(e)) => return Err(anyhow!("IO error: {}", e)),
        Some(Ok(l)) => l.len(),
    };
    let mut totals: Box<[u32]> = vec![0; reading_size].into_boxed_slice();
    let mut num_lines = 0u32;
    for line in lines {
        let line = line?;
        num_lines += 1;

        if line.len() != reading_size {
            return Err(anyhow!("Mismatched line lengths"));
        }
        for (idx, input_char) in line.as_bytes().iter().enumerate() {
            match input_char {
                b'1' => totals[idx] += 1,
                b'0' => {}
                _ => return Err(anyhow!("Input line was not binary")),
            }
        }
    }
    let mut gamma = 0u32;
    let mut epsilon = 0u32;
    for (idx, total) in totals.iter().enumerate() {
        let shift: usize = reading_size - idx - 1;
        if *total > num_lines / 2 {
            gamma |= 1 << shift;
        } else {
            epsilon |= 1 << shift;
        }
    }
    let out = gamma * epsilon;
    Ok(format!("{}", out))
}
