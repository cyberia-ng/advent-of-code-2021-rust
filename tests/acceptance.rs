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
test_day!(day6_2, 6, 2, day6::part2);
test_day!(day7_1, 7, 1, day7::part1);
test_day!(day7_2, 7, 2, day7::part2);
test_day!(day8_1, 8, 1, day8::part1);
test_day!(day8_2, 8, 2, day8::part2);
