pub struct Day4;

use std::collections::VecDeque;

impl Day4 {
    pub fn run_all() {
        let test_input = include_str!("test.txt");
        let real_input = include_str!("input.txt");
        println!("Part 1 (test): {}", Self::part_1(test_input));
        println!("Part 1 (real): {}", Self::part_1(real_input));

        println!("Part 2 (test): {}", Self::part_2(test_input));
        println!("Part 2 (real): {}", Self::part_2(real_input));
    }

    pub fn part_1(input: &str) -> u32 {
        let base: u32 = 2;
        input.lines()
            .map(Self::parse_line)
            .map(|(winning, mine)| Self::count_winners(winning, mine))
            .filter(|count| count > &0)
            .map(|count| base.pow(count as u32 - 1))
            .sum()
    }

    pub fn part_2(input: &str) -> u32 {
        let base: u32 = 2;
        let counts = input.lines()
            .map(Self::parse_line)
            .map(|(winning, mine)| Self::count_winners(winning, mine));

        let mut multipliers = vec![1; counts.clone().count()];
        let mut sum = 0;
        for (i, count) in counts.enumerate() {
            let multiplier = multipliers[i];
            sum += multiplier;
            for offset in 1..=count {
                let index = i + offset;
                multipliers[index] += multiplier;
            }
        }

        sum as u32
    }

    fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
        let mut split = line.split(": ")
            .last()
            .expect("oops no colon")
            .split(" | ");

        let winning_numbers: Vec<&str> =
            split.next()
                .expect("oops no winning numbers")
                .split(' ')
                .filter(|entry| !entry.is_empty())
                .collect();

        let my_numbers: Vec<&str> =
            split.last()
                .expect("oops no my numbers")
                .split(' ')
                .filter(|entry| !entry.is_empty())
                .collect();

        (winning_numbers, my_numbers)
    }

    fn count_winners(
        winning_numbers: Vec<&str>,
        my_numbers: Vec<&str>
    ) -> usize {
        my_numbers.iter()
            .filter(|my_num| winning_numbers.contains(my_num))
            .count()
    }
}

pub struct Doubling {
    pub duration: u32,
    pub strength: u32,
}