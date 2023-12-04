use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve1() -> usize {
    let file = File::open("src/day4/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut points = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let seperated: Vec<&str> = line.split(|x| (x == ':') || (x == '|')).collect();

        let winning_numbers_str: Vec<&str> = (*seperated.get(1).unwrap()).trim().split(' ').collect();
        let own_numbers_str: Vec<&str> = (*seperated.get(2).unwrap()).trim().split(' ').collect();

        let mut winning_numbers = Vec::new();
        let mut own_numbers = Vec::new();

        let mut matching = 0;

        for num_str in winning_numbers_str {
            if num_str != "" {
                let num = usize::from_str_radix(num_str, 10).unwrap();
                winning_numbers.push(num);
            }
        }

        for num_str in own_numbers_str {
            if num_str != "" {
                let num = usize::from_str_radix(num_str, 10).unwrap();
                own_numbers.push(num);
            }
        }

        for num in own_numbers {
            if winning_numbers.contains(&num) {
                matching += 1;
            }
        }

        if matching != 0 {
            points += 2usize.pow(matching - 1);
        }
    }

    points
}

pub fn solve2() -> usize {
    let file = File::open("src/day4/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut copies_amount = [1; 199];
    let mut line_index = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let seperated: Vec<&str> = line.split(|x| (x == ':') || (x == '|')).collect();
        
        let winning_numbers_str: Vec<&str> = (*seperated.get(1).unwrap()).trim().split(' ').collect();
        let own_numbers_str: Vec<&str> = (*seperated.get(2).unwrap()).trim().split(' ').collect();

        let mut winning_numbers = Vec::new();
        let mut own_numbers = Vec::new();

        let mut matching = 0;

        for num_str in winning_numbers_str {
            if num_str != "" {
                let num = usize::from_str_radix(num_str, 10).unwrap();
                winning_numbers.push(num);
            }
        }

        for num_str in own_numbers_str {
            if num_str != "" {
                let num = usize::from_str_radix(num_str, 10).unwrap();
                own_numbers.push(num);
            }
        }

        for num in own_numbers {
            if winning_numbers.contains(&num) {
                matching += 1;
            }
        }

        for i in (line_index+1)..(line_index+matching+1) {
            copies_amount[i] += copies_amount[line_index];
        }

        line_index += 1;
    }

    let mut total = 0;

    for x in copies_amount {
        total += x;
    }

    total
}