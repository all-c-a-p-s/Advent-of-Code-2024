const INPUT: &str = "./input02.txt";

fn is_increasing(v: &Vec<i32>) -> bool {
    for i in 0..v.len() - 1 {
        if v[i + 1] <= v[i] {
            return false;
        }
    }
    true
}

fn max_diff(v: &Vec<i32>) -> i32 {
    v.iter()
        .take(v.len() - 1)
        .enumerate()
        .fold(0, |acc, (i, x)| {
            if (v[i + 1] - x).abs() > acc {
                (v[i + 1] - x).abs()
            } else {
                acc
            }
        })
}

fn is_safe(l: &Vec<i32>) -> bool {
    (is_increasing(l) || is_increasing(&l.iter().rev().map(|x| x.to_owned()).collect()))
        && max_diff(l) <= 3
}

fn get_data() -> Vec<Vec<i32>> {
    let f = std::fs::read_to_string(INPUT).unwrap();
    let data: Vec<Vec<i32>> = f
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    data
}

pub fn part_one() -> i32 {
    let data = get_data();

    data.iter()
        .filter(|x| is_safe(*x))
        .map(|x| x.clone())
        .collect::<Vec<Vec<i32>>>()
        .len() as i32
}

fn is_within_tolerance(l: &Vec<i32>) -> bool {
    for rm in 0..l.len() {
        let v = [&l[0..rm], &l[rm + 1..]].concat();
        if is_safe(&v) {
            return true;
        }
    }
    return false;
}

pub fn part_two() -> i32 {
    let data = get_data();
    data.iter()
        .filter(|x| is_within_tolerance(*x) || is_safe(*x))
        .map(|x| x.clone())
        .collect::<Vec<Vec<i32>>>()
        .len() as i32
}
