use advent_of_code_2021_rust::*;
use std::fs::{read_to_string, File};
use std::io::BufReader;

macro_rules! test_day {
    ($name:ident, $day:expr, $part:expr, $function:expr) => {
        #[test]
        fn $name() {
            let input = File::open(format!("tests/fixtures/day-{}/input.txt", $day)).unwrap();
            let expected_output = read_to_string(format!(
                "tests/fixtures/day-{}/expected-part-{}.txt",
                $day, $part
            ))
            .unwrap();
            let output = $function(BufReader::new(input)).unwrap();
            assert_eq!(output, expected_output.trim_end());
        }
    };
}

test_day!(day04_1, 4, 1, day04::part1);
test_day!(day06_1, 6, 1, day06::part1);
test_day!(day06_2, 6, 2, day06::part2);
test_day!(day07_1, 7, 1, day07::part1);
test_day!(day07_2, 7, 2, day07::part2);
test_day!(day08_1, 8, 1, day08::part1);
test_day!(day08_2, 8, 2, day08::part2);
test_day!(day09_1, 9, 1, day09::part1);
test_day!(day09_2, 9, 2, day09::part2);
test_day!(day10_1, 10, 1, day10::part1);
test_day!(day10_2, 10, 2, day10::part2);
test_day!(day11_1, 11, 1, day11::part1);
test_day!(day11_2, 11, 2, day11::part2);
