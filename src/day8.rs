use anyhow::anyhow;
use std::io::BufRead;

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let mut total = 0u32;
    for input_row in parse_input(input) {
        let input_row = input_row?;
        for output_word in input_row.output.iter() {
            match output_word.len() {
                2 | 3 | 4 | 7 => total += 1,
                5 | 6 => {}
                _ => return Err(anyhow!("unexpected length")),
            }
        }
    }
    Ok(format!("{}", total))
}

pub fn part2(input: impl BufRead) -> anyhow::Result<String> {
    let mut total = 0u32;
    for input_row in parse_input(input) {
        let input_row = input_row?;
        eprintln!("{:?}", input_row);
        let mut possibilities = Possibilities::new();
        for observed_word in input_row.observations.iter() {
            possibilities.update(&observed_word);
            eprintln!("{:?}", possibilities);
        }
        possibilities.reduce();
        eprintln!("{:?}", possibilities);
        let number = input_row
            .output
            .iter()
            .map(|word| possibilities.decode(&word))
            .fold(Ok(0u32), |number, digit| -> anyhow::Result<_> {
                Ok(number? * 10 + (digit? as u32))
            })?;
        total += number;
    }
    Ok(format!("{}", total))
}

#[derive(Debug)]
struct Possibilities([u8; 7]);

impl Possibilities {
    fn new() -> Self {
        Self([0x7F; 7])
    }

    fn update(&mut self, received: &[u8]) {
        let (mask, anti_mask) = get_attestations(received);
        for (received_wire, wire_possibilities) in self.0.iter_mut().enumerate() {
            if received.iter().any(|r| *r as usize == received_wire) {
                *wire_possibilities &= mask;
            } else {
                *wire_possibilities &= anti_mask;
            }
        }
    }

    fn reduce(&mut self) {
        loop {
            let mut confirmed = 0u8;
            for wire_possibilities in self.0.iter() {
                if is_single_bit(*wire_possibilities) {
                    confirmed |= wire_possibilities;
                }
            }
            if confirmed >= 0x7F {
                break;
            }
            let mut changed = false;
            for wire_possibilities in self.0.iter_mut() {
                if !is_single_bit(*wire_possibilities) {
                    let new = *wire_possibilities & !confirmed;
                    if new != *wire_possibilities {
                        changed = true;
                        *wire_possibilities = new;
                    }
                }
            }
            if !changed {
                break;
            }
        }
    }

    fn decode(&self, received: &[u8]) -> anyhow::Result<u8> {
        let mut real_wires = received
            .into_iter()
            .map(|received| {
                let targets = self.0[*received as usize];
                if !is_single_bit(targets) {
                    Err(anyhow!("Ambiguous wire"))
                } else {
                    Ok(targets.trailing_zeros())
                }
            })
            .collect::<anyhow::Result<Vec<_>, _>>()?;
        real_wires.sort_unstable();
        let digit = match &real_wires[..] {
            &[0, 1, 2, 4, 5, 6] => 0,
            &[2, 5] => 1,
            &[0, 2, 3, 4, 6] => 2,
            &[0, 2, 3, 5, 6] => 3,
            &[1, 2, 3, 5] => 4,
            &[0, 1, 3, 5, 6] => 5,
            &[0, 1, 3, 4, 5, 6] => 6,
            &[0, 2, 5] => 7,
            &[0, 1, 2, 3, 4, 5, 6] => 8,
            &[0, 1, 2, 3, 5, 6] => 9,
            _ => return Err(anyhow!("Unknown digit {:?}", real_wires)),
        };
        Ok(digit)
    }
}

fn bits<'a>(vals: impl IntoIterator<Item = &'a u8>) -> u8 {
    vals.into_iter().fold(0u8, |t, x| t | 1 << x)
}

fn is_single_bit(item: u8) -> bool {
    return item != 0 && item & (item - 1) == 0;
}

fn get_attestations(received: &[u8]) -> (u8, u8) {
    match received.len() {
        2 => (bits(&[2, 5]), bits(&[0, 1, 3, 4, 6])), // digit 1
        3 => (bits(&[0, 2, 5]), bits(&[1, 3, 4, 6])), // digit 7
        4 => (bits(&[1, 2, 3, 5]), bits(&[0, 4, 6])), // digit 4
        5 => (0x7F, bits(&[1, 2, 4, 5])), // digits 2, 3, or 5, which share wires 0, 3, and 6
        6 => (0x7F, bits(&[2, 3, 4])),    // digits 0, 6, or 9, which share wires 0, 2, 5 and 6
        7 => (0x7F, 0x00),                // digit 8
        _ => panic!("Bad received: {:?}", received),
    }
}

#[derive(Debug)]
struct InputRow {
    observations: Vec<Vec<u8>>,
    output: Vec<Vec<u8>>,
}

fn parse_input<'input>(
    input: impl BufRead + 'input,
) -> impl Iterator<Item = anyhow::Result<InputRow>> + 'input {
    input.lines().map(|line| {
        let line = line?;
        let line_parts: Vec<&str> = line.split(" | ").take(2).collect::<Vec<_>>();
        if line_parts.len() != 2 {
            return Err(anyhow!("Bad line"));
        }
        let observations: Vec<Vec<_>> = line_parts[0]
            .split(' ')
            .map(|word| word.bytes().map(|b| b - b'a').collect())
            .collect();
        let output: Vec<Vec<_>> = line_parts[1]
            .split(' ')
            .map(|word| word.bytes().map(|b| b - b'a').collect())
            .collect();
        Ok(InputRow {
            observations,
            output,
        })
    })
}
