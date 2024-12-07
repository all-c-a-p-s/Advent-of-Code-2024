const INPUT: &str = include_str!("./input07.txt");

struct Line {
    target: i128,
    nums: Vec<i128>,
}

fn score(target: i128, nums: &[i128]) -> i128 {
    if nums.len() == 1 {
        return if nums[0] == target { target } else { 0 };
    } else if nums[0] > target {
        return 0;
    }
    let mut v1: Vec<i128> = Vec::with_capacity(nums.len() - 1);
    v1.push(nums[0] * nums[1]);
    v1.extend_from_slice(&nums[2..]);

    let mut v2: Vec<i128> = Vec::with_capacity(nums.len() - 1);
    v2.push(nums[0] + nums[1]);
    v2.extend_from_slice(&nums[2..]);

    std::cmp::max(score(target, &v1), score(target, &v2))
}

fn concat(x: i128, y: i128) -> i128 {
    (format!("{}", x) + format!("{}", y).as_str())
        .parse::<i128>()
        .unwrap()
}

fn score_part_two(target: i128, nums: &[i128]) -> i128 {
    if nums.len() == 1 {
        return if nums[0] == target { target } else { 0 };
    } else if nums[0] > target {
        return 0;
    }
    let mut v1: Vec<i128> = Vec::with_capacity(nums.len() - 1);
    v1.push(nums[0] * nums[1]);
    v1.extend_from_slice(&nums[2..]);

    let mut v2: Vec<i128> = Vec::with_capacity(nums.len() - 1);
    v2.push(nums[0] + nums[1]);
    v2.extend_from_slice(&nums[2..]);

    let mut v3: Vec<i128> = Vec::with_capacity(nums.len() - 1);
    v3.push(concat(nums[0], nums[1]));
    v3.extend_from_slice(&nums[2..]);

    std::cmp::max(
        score_part_two(target, &v1),
        std::cmp::max(score_part_two(target, &v2), score_part_two(target, &v3)),
    )
}
fn get_data() -> Vec<Line> {
    let mut v = Vec::new();
    for line in INPUT.lines() {
        let s: Vec<&str> = line.split(":").collect();
        let n = s[0].parse::<i128>().unwrap();
        let ns = s[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        v.push(Line {
            target: n,
            nums: ns,
        });
    }
    v
}

pub fn part_one() -> i128 {
    let data = get_data();
    data.iter().fold(0, |acc, x| acc + score(x.target, &x.nums))
}

pub fn part_two() -> i128 {
    let data = get_data();
    data.iter()
        .fold(0, |acc, x| acc + score_part_two(x.target, &x.nums))
}
