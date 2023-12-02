use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve1() -> usize {
    let file = File::open("src/day2/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut valid_ids = Vec::new();
    let mut game_id = 1;

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let line_separated: Vec<&str> = line_clone.split(|c| (c == ',') || (c == ';') || (c == ':')).collect();

        let mut game_allowed = true;
        for section in &line_separated {
            let number_value = extract_digit_from_text(&section);

            if section.contains("red") && (number_value > 12) {
                game_allowed = false;
            }
            if section.contains("blue") && (number_value > 14) {
                game_allowed = false;
            }
            if section.contains("green") && (number_value > 13) {
                game_allowed = false;
            }
        }

        if game_allowed {
            valid_ids.push(game_id);
        }

        game_id += 1;
    }
    let mut total = 0;

    for id in valid_ids {
        total += id;
    }
    total
}

pub fn solve2() -> usize {
    let file = File::open("src/day2/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut game_powers = Vec::new();

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let line_separated: Vec<&str> = line_clone.split(|c| (c == ',') || (c == ';') || (c == ':')).collect();

        let mut red_max = 0;
        let mut blue_max = 0;
        let mut green_max = 0;

        for section in &line_separated {
            let number_value = extract_digit_from_text(&section);

            if section.contains("red") && (number_value > red_max) {
                red_max = number_value;
            }
            if section.contains("blue") && (number_value > blue_max) {
                blue_max = number_value;
            }
            if section.contains("green") && (number_value > green_max) {
                green_max = number_value;
            }
        }

        let power = red_max * blue_max * green_max;
        game_powers.push(power);
    }
    let mut total = 0;

    for power in game_powers {
        total += power;
    }
    total
}

fn extract_digit_from_text(s: &str) -> usize {
    let mut digit = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            digit.push(c);
        }
    }

    usize::from_str_radix(digit.as_str(), 10).unwrap()
}