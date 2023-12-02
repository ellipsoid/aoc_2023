pub struct Day2;

use std::collections::HashMap;

impl Day2 {
    pub fn run_all() {
        let test_input = include_str!("test.txt");
        let real_input = include_str!("input.txt");
        println!("Part 1 (test): {}", Self::part_1(test_input));
        println!("Part 1 (real): {}", Self::part_1(real_input));

        println!("Part 2 (test): {}", Self::part_2(test_input));
        println!("Part 2 (real): {}", Self::part_2(real_input));
    }

    pub fn part_1(input: &str) -> u32 {
        let red_limit = 12;
        let green_limit = 13;
        let blue_limit = 14;
        input.lines()
            .filter_map(|line| {
                let (game_id, max_red, max_green, max_blue) = Self::max_amounts_per_line(line);
                if max_red > red_limit || max_green > green_limit || max_blue > blue_limit {
                    None
                } else {
                    Some(game_id)
                }
            }).sum()
    }

    pub fn part_2(input: &str) -> u32 {
        input.lines()
            .map(|line| {
                let (_, max_red, max_green, max_blue) = Self::max_amounts_per_line(line);
                max_red * max_green * max_blue
            }).sum()
    }

    fn max_amounts_per_line(line: &str) -> (u32, u32, u32, u32) {
        let (game_id, reveals) = Self::parse_line(line);
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for reveal in reveals.iter() {
            for (amount, color) in reveal.iter() {
                match *color {
                    "red" => {
                        if *amount > max_red { max_red = *amount; }
                    }
                    "green" => {
                        if *amount > max_green { max_green = *amount; }
                    }
                    "blue" => {
                        if *amount > max_blue { max_blue = *amount; }
                    }
                    _ => {}
                }
            }
        }
        (game_id, max_red, max_green, max_blue)
    }

    pub fn parse_line(line: &str) -> (u32, Vec<Vec<(u32, &str)>>) {
        let parts: Vec<&str> = line.split(": ").collect();

        let game_str = parts.get(0).expect("missing game index str");
        let game_id = Self::parse_game_id(game_str);

        let reveals_str = parts.get(1).expect("missing hands section of str");
        let reveals = Self::parse_reveals(reveals_str);

        (game_id, reveals)
    }

    fn parse_game_id(game_str: &str) -> u32 {
        game_str.split(" ")
            .last()
            .expect("couldn't get game id")
            .parse::<u32>()
            .expect("couldn't parse game id")
    }

    fn parse_reveals(reveals_str: &str) -> Vec<Vec<(u32, &str)>> {
        reveals_str.split("; ")
                    .map(|reveal_str| Self::parse_reveal(reveal_str))
                    .collect()
    }

    fn parse_reveal(reveal_str: &str) -> Vec<(u32, &str)> {
        reveal_str.split(", ").map(|num_color_str| {
            Self::parse_number_and_color(num_color_str)
        }).collect()
    }

    fn parse_number_and_color(chunk: &str) -> (u32, &str) {
        let num_color_parts: Vec<&str> = chunk.split(" ").collect();
        let amount = num_color_parts
            .get(0)
            .expect("missing amount of color")
            .parse::<u32>()
            .expect("couldn't turn color amount into u32");

        let color = num_color_parts.get(1).expect("missing color");

        (amount, color)

    }
}