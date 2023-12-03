pub struct Day3;

use regex::Regex;

impl Day3 {
    pub fn run_all() {
        let test_input = include_str!("test.txt");
        let real_input = include_str!("input.txt");
        println!("Part 1 (test): {}", Self::part_1(test_input));
        println!("Part 1 (real): {}", Self::part_1(real_input));

        println!("Part 2 (test): {}", Self::part_2(test_input));
        println!("Part 2 (real): {}", Self::part_2(real_input));
    }

    pub fn part_1(input: &str) -> u32 {
        let mut result = 0;
        let char_map = Self::parse_input(input);
        let numbers = Self::build_numbers(&char_map);
        for number in numbers.iter() {
            if number.neighbor_indices().iter().any(|(line_index, char_index)| {
                if let Some(line) = char_map.get(*line_index) {
                    if let Some(char_type) = line.get(*char_index) {
                        *char_type == CharType::Symbol || *char_type == CharType::Gear
                    } else { false }
                } else { false }
            }) {
                result += number.amount;
            }
        }

        result
    }

    pub fn part_2(input: &str) -> u32 {
        let mut result = 0;
        let char_map = Self::parse_input(input);
        let numbers = Self::build_numbers(&char_map);

        // find gears, check if coords match neighbors from our number list
        for (line_index, line) in char_map.iter().enumerate() {
            for (char_index, char) in line.iter().enumerate() {
                if *char == CharType::Gear {
                    let matching_numbers: Vec<&Number> = numbers.iter().filter(|number| {
                        number.neighbor_indices().contains(&(line_index, char_index))
                    }).collect();

                    if matching_numbers.len() == 2 {
                        result += matching_numbers.iter().fold(1, |acc, num| acc * num.amount);
                    }
                }
            }
        }

        result
    }

    fn build_numbers(char_map: &[Vec<CharType>]) -> Vec<Number> {
        let mut result = vec![];
        let mut current_digits: String = String::new();
        let mut number_start_index = None;
        for (line_index, line) in char_map.iter().enumerate() {
            for (char_index, char_type) in line.iter().enumerate() {
                if let CharType::Digit(digit) = char_type {
                    current_digits.push(*digit);
                    if number_start_index.is_none() {
                        number_start_index = Some(char_index);
                    }

                    // we're in a number, check next char to see if this is the last digit
                    if let Some(&CharType::Digit(_)) = line.get(char_index + 1) {
                        // digit it not final
                    } else {
                        let start_index = number_start_index.expect("no start index");
                        // this is last digit in number
                        let amount = current_digits.parse::<u32>().expect("parse digits");
                        result.push(
                            Number {
                                amount, 
                                line_index,
                                cell_index_start: start_index,
                                cell_index_end: char_index,
                            }
                        );
                        number_start_index = None;
                        current_digits = String::new();
                    }
                }
            }
        }
        result
    }

    fn parse_input(input: &str) -> Vec<Vec<CharType>> {
        input.lines()
            .map(|line| {
                line.chars()
                    .map(CharType::parse)
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
pub enum CharType {
    Empty,
    Digit(char),
    Gear,
    Symbol,
}

impl CharType {
    pub fn parse(ch: char) -> Self {
        if ch == '.' { CharType::Empty }
        else if ch == '*' { CharType::Gear }
        else if ch.is_ascii_digit()  { CharType::Digit(ch) }
        else { CharType::Symbol }
    }
}

struct Number {
    amount: u32,
    line_index: usize,
    cell_index_start: usize,
    cell_index_end: usize,
}

impl Number {
    pub fn neighbor_indices(&self) -> Vec<(usize, usize)> {
        let first_line_index = if self.line_index == 0 { 0 } else { self.line_index - 1 };
        let last_line_index = self.line_index + 1;

        let first_cell_index = if self.cell_index_start == 0 { 0 } else { self.cell_index_start - 1 };
        let last_cell_index = self.cell_index_end + 1;

        let mut result = vec![];
        for line_index in first_line_index..=last_line_index {
            for cell_index in first_cell_index..=last_cell_index {
                if !self.contains(line_index, cell_index) {
                    result.push((line_index, cell_index));
                }
            }
        }

        result
    }

    pub fn contains(&self, line_index: usize, cell_index: usize) -> bool {
        self.line_index == line_index
            && self.cell_index_start <= cell_index
            && cell_index <= self.cell_index_end
    }
}