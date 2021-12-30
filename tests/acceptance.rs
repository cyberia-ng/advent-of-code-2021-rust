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

test_day!(day4_1, 4, 1, day4::part1);
test_day!(day6_1, 6, 1, day6::part1);
