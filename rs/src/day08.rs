use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./input08.txt");

fn get_antinode_coords(a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32)> {
    let mut possible_coords = Vec::new();
    let ab = (b.0 - a.0, b.1 - a.1);
    let between = if ab.0 % 3 == 0 && ab.1 % 3 == 0 {
        Some((ab.0 / 3, ab.1 / 3))
    } else {
        None
    };
    if between.is_some() {
        possible_coords.push((ab.0, ab.1));
        possible_coords.push((ab.0 * 2, ab.1 * 2));
    }
    possible_coords.push((a.0 - ab.0, a.1 - ab.1));
    possible_coords.push((b.0 + ab.0, b.1 + ab.1));
    possible_coords
}

fn hcf(a: i32, b: i32) -> i32 {
    match b {
        0 => a,
        _ => hcf(b, a % b),
    }
}

fn get_antinode_coords_2(
    a: (i32, i32),
    b: (i32, i32),
    lenx: usize,
    leny: usize,
) -> Vec<(usize, usize)> {
    let mut possible_coords = vec![(a.0 as usize, a.1 as usize)];
    let ab = (b.0 - a.0, b.1 - a.1);
    let h = hcf(ab.0, ab.1);
    let s = (ab.0 / h, ab.1 / h);
    let mut i = 0;
    while is_valid(a.0 - s.0 * i, a.1 - s.1 * i, lenx, leny) {
        possible_coords.push(((a.0 - s.0 * i) as usize, (a.1 - s.1 * i) as usize));
        i += 1;
    }

    let mut j = 0;
    while is_valid(a.0 + s.0 * j, a.1 + s.1 * j, lenx, leny) {
        possible_coords.push(((a.0 + s.0 * j) as usize, (a.1 + s.1 * j) as usize));
        j += 1;
    }
    possible_coords
}

fn get_data() -> Vec<Vec<char>> {
    INPUT.lines().map(|x| x.chars().collect()).collect()
}

fn is_valid(x: i32, y: i32, lenx: usize, leny: usize) -> bool {
    x >= 0 && y >= 0 && x < lenx as i32 && y < leny as i32
}

fn get_frequency_positions(data: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut frequency_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for row in 0..data.len() {
        for column in 0..data[0].len() {
            match data[row][column] {
                '.' => {}
                x if frequency_positions.contains_key(&x) => {
                    let v = frequency_positions.get_mut(&x).unwrap();
                    v.push((row, column));
                }
                y => {
                    let _ = frequency_positions.insert(y, vec![(row, column)]);
                    ()
                }
            };
        }
    }
    frequency_positions
}

pub fn part_one() -> i32 {
    let data = get_data();
    let frequency_positions = get_frequency_positions(&data);
    let mut antinode_positions: HashSet<(usize, usize)> = HashSet::new();
    for (_, v) in frequency_positions {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if j == i {
                    continue;
                }
                let antinode_coords = get_antinode_coords(
                    (v[i].0 as i32, v[i].1 as i32),
                    (v[j].0 as i32, v[j].1 as i32),
                );
                for p in antinode_coords
                    .iter()
                    .filter(|x| is_valid(x.0, x.1, data.len(), data[0].len()))
                {
                    antinode_positions.insert((p.0 as usize, p.1 as usize));
                }
            }
        }
    }
    antinode_positions.len() as i32
}

pub fn part_two() -> i32 {
    let data = get_data();
    let frequency_positions = get_frequency_positions(&data);
    let mut antinode_positions: HashSet<(usize, usize)> = HashSet::new();
    for (_, v) in frequency_positions {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if j == i {
                    continue;
                }
                let antinode_coords = get_antinode_coords_2(
                    (v[i].0 as i32, v[i].1 as i32),
                    (v[j].0 as i32, v[j].1 as i32),
                    data.len(),
                    data[0].len(),
                );
                for p in antinode_coords {
                    antinode_positions.insert(p);
                }
            }
        }
    }

    antinode_positions.len() as i32
}
