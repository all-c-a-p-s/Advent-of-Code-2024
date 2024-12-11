use cached::proc_macro::cached;

const INPUT: &str = include_str!("./input11.txt");

fn get_data() -> Vec<i128> {
    INPUT
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i128>().unwrap())
        .collect()
}

//know x != 0
fn digits(x: i128) -> i128 {
    let (mut y, mut digits) = (1, 0);
    while y <= x {
        y *= 10;
        digits += 1;
    }
    digits
}

fn exp(b: i128, e: i128) -> i128 {
    match e {
        0 => 1,
        _ => b * exp(b, e - 1),
    }
}

fn split(x: i128) -> (i128, i128) {
    let d = digits(x) / 2;
    let a = x / exp(10, d);
    let b = x - a * exp(10, d);
    (a, b)
}

#[cached]
fn dfs(x: i128, depth: i128) -> i128 {
    if depth == 0 {
        return 1;
    }

    if x == 0 {
        return dfs(1, depth - 1);
    }

    if digits(x) % 2 == 0 {
        let s = split(x);
        return dfs(s.0, depth - 1) + dfs(s.1, depth - 1);
    }
    dfs(x * 2024, depth - 1)
}

pub fn part_one() -> i128 {
    let data = get_data();
    data.iter().fold(0, |acc, x| acc + dfs(*x, 25))
}
pub fn part_two() -> i128 {
    let data = get_data();
    data.iter().fold(0, |acc, x| acc + dfs(*x, 75))
}
