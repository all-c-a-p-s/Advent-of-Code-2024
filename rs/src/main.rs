use std::io::{stdout, Write};
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn take_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    print!("Enter day to solve: ");
    let _ = stdout().flush();
    let day = take_int();

    print!("Enter part to solve: ");
    let _ = stdout().flush();
    let part = take_int();

    println!();
    let start = Instant::now();

    print!("Answer: ");
    match (day, part) {
        (1, 1) => println!("{}", day01::part_one()),
        (1, 2) => println!("{}", day01::part_two()),
        (2, 1) => println!("{}", day02::part_one()),
        (2, 2) => println!("{}", day02::part_two()),
        (3, 1) => println!("{}", day03::part_one()),
        (3, 2) => println!("{}", day03::part_two()),
        (4, 1) => println!("{}", day04::part_one()),
        (4, 2) => println!("{}", day04::part_two()),
        (5, 1) => println!("{}", day05::part_one()),
        (5, 2) => println!("{}", day05::part_two()),
        (6, 1) => println!("{}", day06::part_one()),
        (6, 2) => println!("{}", day06::part_two()),
        (7, 1) => println!("{}", day07::part_one()),
        (7, 2) => println!("{}", day07::part_two()),
        (8, 1) => println!("{}", day08::part_one()),
        (8, 2) => println!("{}", day08::part_two()),
        (9, 1) => println!("{}", day09::part_one()),
        (9, 2) => println!("{}", day09::part_two()),
        (10, 1) => println!("{}", day10::part_one()),
        (10, 2) => println!("{}", day10::part_two()),

        _ => panic!("sorry not implemented yet"),
    };

    let duration = start.elapsed();
    println!("Runtime: {:?}", duration);
}
