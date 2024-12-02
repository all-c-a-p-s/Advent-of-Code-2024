const INPUT: &str = include_str!("./input01.txt");

fn count_appearances(t: i32, v: &[i32]) -> i32 {
    v.iter().fold(0, |acc, x| acc + if *x == t { 1 } else { 0 })
}

fn get_lists() -> (Vec<i32>, Vec<i32>) {
    let f = String::from(INPUT);
    let lines: Vec<String> = f.lines().map(|l| l.to_string()).collect();

    let (mut first_list, mut second_list) = (Vec::new(), Vec::new());

    for line in lines {
        let nums: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        first_list.push(nums[0]);
        second_list.push(nums[1]);
    }
    (first_list, second_list)
}

pub fn part_one() -> i32 {
    let (mut first_list, mut second_list) = get_lists();

    first_list.sort();
    second_list.sort();

    first_list
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (second_list[i] - x).abs())
}

pub fn part_two() -> i32 {
    let (first_list, second_list) = get_lists();

    first_list
        .iter()
        .fold(0, |acc, x| acc + count_appearances(*x, &second_list) * x)
}
