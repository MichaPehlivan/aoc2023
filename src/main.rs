mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    //day 1
    let answer1_1 = day1::solve1();
    let answer1_2 = day1::solve2();
    println!("day 1.1: {}", answer1_1);
    println!("day 1.2: {}\n", answer1_2);

    //day 2
    let answer2_1 = day2::solve1();
    let answer2_2 = day2::solve2();
    println!("day 2.1: {}", answer2_1);
    println!("day 2.2: {}\n", answer2_2);

    //day 3
    let answer3_1 = day3::solve1();
    let answer3_2 = day3::solve2();
    println!("day 3.1: {}", answer3_1);
    println!("day 3.2: {}\n", answer3_2);

    //day 4
    let answer4_1 = day4::solve1();
    let answer4_2 = day4::solve2();
    println!("day 4.1: {}", answer4_1);
    println!("day 4.2: {}\n", answer4_2);
}
