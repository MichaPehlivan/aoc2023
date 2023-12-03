use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve1() -> usize {
    let file = File::open("src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut adjacent_coords: Vec<(isize, isize)> = Vec::new();
    let mut vec_input: Vec<(char, (isize, isize))> = Vec::new();

    let mut y = 0;
    for line in reader.lines() {
        let line = line.unwrap().clone();
        let mut x = 0;
        for c in line.chars() {
            vec_input.push((c, (x, y)));

            if !((c == '.') || c.is_ascii_digit() || c.is_whitespace()) {
                adjacent_coords.push((x-1, y-1));
                adjacent_coords.push((x, y-1));
                adjacent_coords.push((x+1, y-1));
                adjacent_coords.push((x-1, y));
                adjacent_coords.push((x+1, y));
                adjacent_coords.push((x-1, y+1));
                adjacent_coords.push((x, y+1));
                adjacent_coords.push((x+1, y+1));
            }
            x += 1;
        }
        y += 1;
    }

    adjacent_coords.retain(|x| {
        vec_input.get(vec_input.iter().position(|c| c.1 == *x).unwrap()).unwrap().0.is_ascii_digit()
    });

    for _ in 0..2 {
        for c in &vec_input {
            if c.0.is_ascii_digit() {
                if adjacent_coords.contains(&(c.1.0-1, c.1.1)) || adjacent_coords.contains(&(c.1.0+1, c.1.1)) {
                    adjacent_coords.push(c.1);
                }
            }
        }
    }

    vec_input.retain(|x| adjacent_coords.contains(&x.1));

    let mut total = 0;
    let mut current_digit = String::from("0");
    let mut last_coords = (-1, 0);
    for c in vec_input {
        //println!("current digit: {}", current_digit);
        if (c.1.0 - last_coords.0 > 1) || (c.1.1 != last_coords.1) {
            let digit_numerical = usize::from_str_radix(&current_digit, 10).unwrap();
            total += digit_numerical;
            current_digit.clear();
        }
        current_digit.push(c.0);
        last_coords = c.1;
    }
    total += usize::from_str_radix(&current_digit, 10).unwrap();

    total
}

pub fn solve2() -> usize {
    let file = File::open("src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut vec_input: Vec<Vec<(char, (isize, isize))>> = Vec::new();
    let mut gear_ratios: Vec<usize> = Vec::new();

    let mut y = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let mut line_vec = Vec::new();
        let mut x = 0;
        for c in line.chars() {
            line_vec.push((c, (x, y)));
            x += 1;
        }
        y += 1;
        vec_input.push(line_vec);
    }

    for line in &vec_input {
        let line_clone = line.clone();
        for c in line_clone {
            if c.0 == '*' {
                let mut adjacent_digits = Vec::new();
                let mut total_adjacent = 0;
                let mut current_search = String::new();

                if line.get((c.1.0-1) as usize).unwrap().0.is_ascii_digit() {
                    current_search.push(line.get((c.1.0-1) as usize).unwrap().0);
                    total_adjacent += 1;
                    
                    if line.get((c.1.0-2) as usize).unwrap().0.is_ascii_digit() {
                        current_search.insert(0, line.get((c.1.0-2) as usize).unwrap().0);

                        if line.get((c.1.0-3) as usize).unwrap().0.is_ascii_digit() {
                            current_search.insert(0, line.get((c.1.0-3) as usize).unwrap().0);
                        }
                    }
                    adjacent_digits.push(current_search.clone());
                    current_search.clear();
                }
                if line.get((c.1.0+1) as usize).unwrap().0.is_ascii_digit() {
                    current_search.push(line.get((c.1.0+1) as usize).unwrap().0);
                    total_adjacent += 1;

                    if line.get((c.1.0+2) as usize).unwrap().0.is_ascii_digit() {
                        current_search.push(line.get((c.1.0+2) as usize).unwrap().0);

                        if line.get((c.1.0+3) as usize).unwrap().0.is_ascii_digit() {
                            current_search.push(line.get((c.1.0+3) as usize).unwrap().0);
                        }
                    }
                    adjacent_digits.push(current_search.clone());
                    current_search.clear();
                }

                let previous_line = vec_input.get((c.1.1-1) as usize).unwrap();
                if previous_line.get(c.1.0 as usize).unwrap().0.is_ascii_digit() {
                    current_search.push(previous_line.get(c.1.0 as usize).unwrap().0);
                    total_adjacent += 1;

                    if previous_line.get((c.1.0-1) as usize).unwrap().0.is_ascii_digit() {
                        current_search.insert(0, previous_line.get((c.1.0-1) as usize).unwrap().0);

                        if previous_line.get((c.1.0+1) as usize).unwrap().0.is_ascii_digit() {
                            current_search.push(previous_line.get((c.1.0+1) as usize).unwrap().0);
                        }
                        else if previous_line.get((c.1.0-2) as usize).unwrap().0.is_ascii_digit() {
                            current_search.insert(0, previous_line.get((c.1.0-2) as usize).unwrap().0);
                        }
                    }
                    else if previous_line.get((c.1.0+1) as usize).unwrap().0.is_ascii_digit() {
                        current_search.push(previous_line.get((c.1.0+1) as usize).unwrap().0);

                        if previous_line.get((c.1.0+2) as usize).unwrap().0.is_ascii_digit() {
                            current_search.push(previous_line.get((c.1.0+2) as usize).unwrap().0);
                        }
                    }
                    adjacent_digits.push(current_search.clone());
                    current_search.clear();
                }
                else {
                    if previous_line.get((c.1.0-1) as usize).unwrap().0.is_ascii_digit() {
                        current_search.push(previous_line.get((c.1.0-1) as usize).unwrap().0);
                        total_adjacent += 1;

                        if previous_line.get((c.1.0-2) as usize).unwrap().0.is_ascii_digit() {
                            current_search.insert(0, previous_line.get((c.1.0-2) as usize).unwrap().0);

                            if previous_line.get((c.1.0-3) as usize).unwrap().0.is_ascii_digit() {
                                current_search.insert(0, previous_line.get((c.1.0-3) as usize).unwrap().0);
                            }
                        }
                        adjacent_digits.push(current_search.clone());
                        current_search.clear();
                    }
                    if previous_line.get((c.1.0+1) as usize).unwrap().0.is_ascii_digit() {
                        current_search.push(previous_line.get((c.1.0+1) as usize).unwrap().0); 
                        total_adjacent += 1;

                        if previous_line.get((c.1.0+2) as usize).unwrap().0.is_ascii_digit() {    
                            current_search.push(previous_line.get((c.1.0+2) as usize).unwrap().0);

                            if previous_line.get((c.1.0+3) as usize).unwrap().0.is_ascii_digit() {
                                current_search.push(previous_line.get((c.1.0+3) as usize).unwrap().0);
                            }
                        }
                        adjacent_digits.push(current_search.clone());
                        current_search.clear();
                    }
                }

                let next_line = vec_input.get((c.1.1+1) as usize).unwrap();
                if next_line.get(c.1.0 as usize).unwrap().0.is_ascii_digit() {
                    current_search.push(next_line.get(c.1.0 as usize).unwrap().0);
                    total_adjacent += 1;

                    if next_line.get((c.1.0-1) as usize).unwrap().0.is_ascii_digit() {
                        current_search.insert(0, next_line.get((c.1.0-1) as usize).unwrap().0);

                        if next_line.get((c.1.0+1) as usize).unwrap().0.is_ascii_digit() {
                            current_search.push(next_line.get((c.1.0+1) as usize).unwrap().0);
                        }
                        else if next_line.get((c.1.0-2) as usize).unwrap().0.is_ascii_digit() {
                            current_search.insert(0, next_line.get((c.1.0-2) as usize).unwrap().0);
                        }
                    }
                    else if next_line.get((c.1.0+1) as usize).unwrap().0.is_ascii_digit() {
                        current_search.push(next_line.get((c.1.0+1) as usize).unwrap().0);

                        if next_line.get((c.1.0+2) as usize).unwrap().0.is_ascii_digit() {
                            current_search.push(next_line.get((c.1.0+2) as usize).unwrap().0);
                        }
                    }
                    adjacent_digits.push(current_search.clone());
                    current_search.clear();
                }
                else {
                    if next_line.get((c.1.0-1) as usize).unwrap().0.is_ascii_digit() {
                        current_search.push(next_line.get((c.1.0-1) as usize).unwrap().0);
                        total_adjacent += 1;

                        if next_line.get((c.1.0-2) as usize).unwrap().0.is_ascii_digit() {
                            current_search.insert(0, next_line.get((c.1.0-2) as usize).unwrap().0);

                            if next_line.get((c.1.0-3) as usize).unwrap().0.is_ascii_digit() {
                                current_search.insert(0, next_line.get((c.1.0-3) as usize).unwrap().0);
                            }
                        }
                        adjacent_digits.push(current_search.clone());
                        current_search.clear();
                    }
                    if next_line.get((c.1.0+1) as usize).unwrap().0.is_ascii_digit() {
                        current_search.push(next_line.get((c.1.0+1) as usize).unwrap().0);
                        total_adjacent += 1;

                        if next_line.get((c.1.0+2) as usize).unwrap().0.is_ascii_digit() {
                            current_search.push(next_line.get((c.1.0+2) as usize).unwrap().0);

                            if next_line.get((c.1.0+3) as usize).unwrap().0.is_ascii_digit() {
                                current_search.push(next_line.get((c.1.0+3) as usize).unwrap().0);
                            }

                        }
                        adjacent_digits.push(current_search.clone());
                        current_search.clear();
                    }
                }

                if total_adjacent == 2 {
                    let first_digit = usize::from_str_radix(adjacent_digits.get(0).unwrap(), 10).unwrap();
                    let second_digit = usize::from_str_radix(adjacent_digits.get(1).unwrap(), 10).unwrap();
                    let ratio = first_digit * second_digit;
                    gear_ratios.push(ratio);
                }
            }
        }
    }

    let mut total = 0;
    for ratio in gear_ratios {
        total += ratio;
    }
    total
}
