const INPUT: &str = include_str!("./input09.txt");

fn get_data() -> Vec<i128> {
    let nums: Vec<char> = INPUT.trim().chars().collect();
    let mut res = Vec::new();
    for i in 0..nums.len() {
        let n = String::from(nums[i]).parse::<i128>().unwrap();
        match i % 2 {
            0 => {
                for _ in 0..n {
                    res.push(i as i128 / 2);
                }
            }
            1 => {
                for _ in 0..n {
                    res.push(-1)
                }
            }
            _ => unreachable!(),
        }
    }
    res
}

fn count_dots(v: &Vec<i128>) -> usize {
    v.iter()
        .fold(0, |acc, x| acc + if *x == -1 { 1 } else { 0 })
}

fn get_empty_indices(v: &Vec<i128>) -> Vec<usize> {
    let mut r = Vec::new();
    for i in 0..v.len() {
        if v[i] == -1 {
            r.push(i);
        }
    }
    r
}

pub fn part_one() -> i128 {
    let mut total = 0;

    let mut v = get_data();
    let mut nums = 0;
    let mut dots = count_dots(&v);

    let empties = get_empty_indices(&v);
    let mut end_index = 0;

    for end in (0..v.len()).rev() {
        match v[end] {
            -1 => dots -= 1,
            _ => nums += 1,
        };
        if nums == dots {
            end_index = end;
            break;
        } else if nums > dots {
            end_index = end - 1;
            break;
        }
    }

    let mut count = 0;
    for i in (end_index..v.len()).rev() {
        if v[i] == -1 {
            continue;
        }
        v[empties[count]] = v[i];
        v[i] = -1;
        count += 1;
    }

    for i in 0..v.len() {
        if v[i] == -1 {
            break;
        }
        total += i as i128 * v[i];
    }
    total
}

pub fn part_two() -> i128 {
    let (mut total, mut file_size) = (0, 0);
    let mut v = get_data();
    let mut prev = v[v.len() - 1];

    for i in (0..v.len()).rev() {
        if v[i] != prev {
            //look for first gap that will fit
            let mut gap_size = 0;
            for j in 0..i + 1 {
                if v[j] != -1 {
                    gap_size = 0;
                } else {
                    gap_size += 1;
                }

                if gap_size >= file_size {
                    //fill numbers in to the gap and replace
                    for k in 0..gap_size {
                        v[j - k] = prev;
                        v[i + k + 1] = -1;
                    }
                    break;
                }
            }
            //reset variables
            file_size = 0;
            prev = v[i];
            if v[i] != -1 {
                file_size = 1;
            }
        } else {
            file_size += 1;
        }
    }

    for i in 0..v.len() {
        if v[i] != -1 {
            total += i as i128 * v[i];
        }
    }
    total
}
