pub struct Day1;

use regex::Regex;

impl Day1 {
    pub fn run_all() {
        let test_input = include_str!("test.txt");
        let real_input = include_str!("input.txt");
        println!("Part 1 (test): {}", Self::part_1(test_input));
        println!("Part 1 (real): {}", Self::part_1(real_input));

        let test_input_2 = include_str!("test2.txt");
        println!("Part 2 (test): {}", Self::part_2(test_input_2));
        println!("Part 2 (real): {}", Self::part_2(real_input));
    }

    pub fn part_1(input: &str) -> u32 {
        input.lines().map(|line| {
                Self::part_1_digits(line)
            })
            .map(|digits| { 
                Self::combine_to_2_digit_number(digits).unwrap_or(0)
            })
            .sum()
    }

    pub fn part_2(input: &str) -> u32 {
        input.lines().map(|line| {
                Self::part_2_digits(line)
            })
            .map(|digits| { 
                Self::combine_to_2_digit_number(digits).unwrap_or(0)
            })
            .sum()
    }

    fn part_2_digits(line: &str) -> Vec<u32> {
        if let Ok(re) = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)") {
            re.find_iter(line).map(|regex_match| {
                let match_string = regex_match.as_str();
                match match_string {
                    "1" | "one" => 1,
                    "2" | "two" => 2,
                    "3" | "three" => 3,
                    "4" | "four" => 4,
                    "5" | "five" => 5,
                    "6" | "six" => 6,
                    "7" | "seven" => 7,
                    "8" | "eight" => 8,
                    "9" | "nine" => 9,
                    _ => 0
                }
            }).collect()
        } else { vec![] }
    }

    fn part_1_digits(line: &str) -> Vec<u32> {
        line.chars()
            .filter_map(|char| char.to_digit(10))
            .collect()
    }

    fn combine_to_2_digit_number(digits: Vec<u32>) -> Option<u32> {
        Some(
            digits.first()? * 10 + digits.last()?
        )
    }
}