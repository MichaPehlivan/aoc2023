mod day1;
mod day2;

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
}
