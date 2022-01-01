use std::io::BufRead;

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let mut total_score = 0u32;
    for line in input.lines() {
        let validation = validate_line(&line?);
        if let Validation::Corrupt(b) = validation {
            total_score += part1_score(&b);
        }
    }
    Ok(format!("{}", total_score))
}

pub fn part2(input: impl BufRead) -> anyhow::Result<String> {
    let mut scores = Vec::new();
    for line in input.lines() {
        let validation = validate_line(&line?);
        if let Validation::Incomplete(exp) = validation {
            let score = part2_score(&exp);
            scores.push(score);
        }
    }
    scores.sort_unstable();
    let median = scores[scores.len() / 2];
    Ok(format!("{}", median))
}

#[derive(Debug, Clone, Copy)]
enum Action {
    PushExpectedClosing(char),
    VerifyExpectedClosing,
}
use Action::*;

fn handle(bracket: &char) -> Action {
    match bracket {
        '(' => PushExpectedClosing(')'),
        ')' => VerifyExpectedClosing,
        '[' => PushExpectedClosing(']'),
        ']' => VerifyExpectedClosing,
        '{' => PushExpectedClosing('}'),
        '}' => VerifyExpectedClosing,
        '<' => PushExpectedClosing('>'),
        '>' => VerifyExpectedClosing,
        _ => panic!("handle() called with invalid char"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Validation {
    Incomplete(Vec<char>),
    Corrupt(char),
    Valid,
}

fn validate_line(line: &str) -> Validation {
    let mut expected_closing: Vec<char> = Vec::new();
    for bracket in line.chars() {
        let action = handle(&bracket);
        match action {
            PushExpectedClosing(b) => expected_closing.push(b),
            VerifyExpectedClosing => {
                if Some(bracket) != expected_closing.pop() {
                    return Validation::Corrupt(bracket);
                }
            }
        }
    }
    if expected_closing.is_empty() {
        Validation::Valid
    } else {
        Validation::Incomplete(expected_closing)
    }
}

fn part1_score(bracket: &char) -> u32 {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("part1_score() called with invalid char"),
    }
}

fn part2_score(expected_closing: &[char]) -> u64 {
    let mut total = 0;
    for bracket in expected_closing.iter().rev() {
        total *= 5;
        total += match bracket {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!(
                "part2_score called with invalid char in expected_closing: {:?}",
                expected_closing
            ),
        };
    }
    total
}
