use std::{fs::File, io::{BufReader, BufRead}};

pub fn solve1() -> usize {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut numbers: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        let mut digit = String::new();
        for c in line_clone.chars() {
            if c.is_ascii_digit() {
                digit.push(c);
                break;
            }
        }
        for c in line_clone.chars().rev() {
            if c.is_ascii_digit() {
                digit.push(c);
                break;
            }
        }

        let digit_as_num = usize::from_str_radix(digit.as_str(), 10).unwrap();
        numbers.push(digit_as_num);
    }
    
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

pub fn solve2() -> usize {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut numbers: Vec<usize> = Vec::new();

    let valid_strings = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", 
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();

        //generate vec of all substrings
        let mut slice_list = Vec::new();
        let len = line_clone.len();
        for i in 0..len {
            for j in 0..5 {
                if i + j < len {
                    let substring = &line_clone.as_str()[i..i+j+1];
                    if valid_strings.contains(&substring) { //check if substring is a valid number representation
                        slice_list.push(substring);
                    }
                }
            }
        }

        let number_list = slice_list.iter().map(|s| {
            let mut new = *s;
            if new == "zero" {new = "0"};
            if new == "one" {new = "1"};
            if new == "two" {new = "2"};
            if new == "three" {new = "3"};
            if new == "four" {new = "4"};
            if new == "five" {new = "5"};
            if new == "six" {new = "6"};
            if new == "seven" {new = "7"};
            if new == "eight" {new = "8"};
            if new == "nine" {new = "9"};
            new
        }).collect::<Vec<&str>>();

        let mut number_string = String::new();
        number_string.push_str(number_list.first().unwrap());
        number_string.push_str(number_list.last().unwrap());

        let digit_as_num = usize::from_str_radix(number_string.as_str(), 10).unwrap();
        numbers.push(digit_as_num);
    }
    
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}
