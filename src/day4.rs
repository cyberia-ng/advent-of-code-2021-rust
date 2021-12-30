use anyhow::anyhow;
use std::{
    fmt::Debug,
    io::BufRead,
    mem::{transmute, MaybeUninit},
};

type Int = u8;

#[derive(Debug, Clone, Copy)]
struct BingoNumber {
    marked: bool,
    number: Int,
}

const BOARD_HEIGHT: usize = 5;
const BOARD_WIDTH: usize = 5;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

#[derive(Clone)]
struct Board {
    numbers: [BingoNumber; BOARD_SIZE],
    score: Option<u32>,
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in Self::rows(&self.numbers) {
            for bn in row.iter() {
                if bn.marked {
                    write!(f, "{:>2}* ", bn.number)?;
                } else {
                    write!(f, "{:>2}  ", bn.number)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn new(numbers: [Int; BOARD_SIZE]) -> Self {
        Board {
            numbers: numbers.map(|n| BingoNumber {
                marked: false,
                number: n,
            }),
            score: None,
        }
    }

    /// Returns a boolean indicating if the state changed
    fn draw_number(&mut self, number: Int) -> bool {
        if self.score.is_some() {
            return false;
        }

        for bingo_number in self.numbers.iter_mut() {
            if bingo_number.number == number {
                bingo_number.marked = true;
            }
        }

        for row in Self::rows(&self.numbers) {
            if row.iter().all(|bn| bn.marked) {
                self.score = Some(number as u32 * self.sum_unmarked());
                return true;
            }
        }

        for col in Self::cols(&self.numbers) {
            if col.iter().all(|bn| bn.marked) {
                self.score = Some(number as u32 * self.sum_unmarked());
                return true;
            }
        }

        return false;
    }

    fn sum_unmarked(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|bn| !bn.marked)
            .fold(0u32, |total, item| total + item.number as u32)
    }

    fn rows(numbers: &[BingoNumber]) -> impl Iterator<Item = [BingoNumber; BOARD_WIDTH]> + '_ {
        ({ 0..BOARD_HEIGHT }).map(|row_idx| {
            let start = row_idx * BOARD_WIDTH;
            let end = (row_idx + 1) * BOARD_WIDTH;
            numbers[start..end].try_into().unwrap()
        })
    }

    fn cols(numbers: &[BingoNumber]) -> impl Iterator<Item = [BingoNumber; BOARD_HEIGHT]> + '_ {
        ({ 0..BOARD_WIDTH }).map(|col_idx| {
            let mut out: [MaybeUninit<BingoNumber>; BOARD_HEIGHT] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for (row_idx, item) in out.iter_mut().enumerate() {
                let _ = *item.write(numbers[row_idx * BOARD_WIDTH + col_idx]);
            }
            unsafe { transmute(out) }
        })
    }
}

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let input = parse_input(input)?;
    let mut state = input.boards;
    for drawn_number in input.random_draw.into_iter() {
        for board in state.iter_mut() {
            board.draw_number(drawn_number);
            if let Some(score) = board.score {
                return Ok(format!("{}", score));
            }
        }
    }
    Err(anyhow!("No winners :("))
}

pub fn part2(input: impl BufRead) -> anyhow::Result<String> {
    let input = parse_input(input)?;
    let mut state = input.boards.into_boxed_slice();
    let mut last_complete_idx: Option<usize> = None;
    for drawn_number in input.random_draw.into_iter() {
        for (board_idx, board) in state.iter_mut().enumerate() {
            if board.draw_number(drawn_number) {
                last_complete_idx = Some(board_idx);
            }
        }
    }
    if let Some(idx) = last_complete_idx {
        Ok(format!("{}", state[idx].score.unwrap()))
    } else {
        Err(anyhow!("No winners :("))
    }
}

#[derive(Debug)]
struct Input {
    random_draw: Vec<Int>,
    boards: Vec<Board>,
}

fn parse_input(input: impl BufRead) -> anyhow::Result<Input> {
    let lines = input.lines().collect::<Result<Vec<_>, _>>()?;
    let mut paras = lines
        .split(|line| line.is_empty())
        .map(|lines| lines.join(" "));
    let random_draw = paras
        .next()
        .ok_or(anyhow!("Not enough input"))?
        .split(',')
        .map(|n| n.parse::<Int>())
        .collect::<Result<Vec<_>, _>>()?;
    let boards = paras
        .map(|para| -> anyhow::Result<_> {
            let numbers: [Int; BOARD_SIZE] = para
                .split(' ')
                .filter(|word| !word.is_empty())
                .map(|n| n.parse::<Int>())
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|_| anyhow!("Board size mismatch"))?;
            Ok(Board::new(numbers))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input {
        random_draw,
        boards,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn board_cols() {
        let numbers = ({ 0..(BOARD_SIZE as Int) }).collect::<Vec<_>>();
        let board = Board::new(numbers.try_into().unwrap());
        let cols = Board::cols(&board.numbers).collect::<Vec<_>>();
        assert_eq!(cols[0].map(|bn| bn.number), [0, 5, 10, 15, 20]);
        assert_eq!(cols[4].map(|bn| bn.number), [4, 9, 14, 19, 24]);
    }
}
