use std::collections::HashSet;

const INPUT: &str = include_str!("./input06.txt");

fn get_map() -> Vec<Vec<char>> {
    INPUT.lines().map(|x| x.chars().collect()).collect()
}

fn find_guard(map: &[Vec<char>]) -> (usize, usize) {
    //only called once at the start
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                return (j, i);
            }
        }
    }
    panic!("no guard in map")
}

fn is_valid(x: i32, y: i32, lenx: usize, leny: usize) -> bool {
    x >= 0 && y >= 0 && x < lenx as i32 && y < leny as i32
}

pub fn part_one() -> i32 {
    let mut positions_visited = 1;
    let mut map = get_map();
    let (mut guard_position, mut dir) = (find_guard(&map), (0, -1));
    loop {
        map[guard_position.1][guard_position.0] = 'X';
        let position_in_front = (
            guard_position.0 as i32 + dir.0,
            guard_position.1 as i32 + dir.1,
        );

        if !is_valid(
            position_in_front.0,
            position_in_front.1,
            map[0].len(),
            map[1].len(),
        ) {
            break;
        }

        if map[position_in_front.1 as usize][position_in_front.0 as usize] == '#' {
            dir = successon_dir(dir);
            continue;
        }

        guard_position = (
            (guard_position.0 as i32 + dir.0) as usize,
            (guard_position.1 as i32 + dir.1) as usize,
        );

        if map[guard_position.1][guard_position.0] != 'X' {
            positions_visited += 1;
        }
    }
    positions_visited
}

fn successon_dir(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

fn creates_loop(map: &[Vec<char>]) -> bool {
    let mut hash_set: HashSet<(usize, usize, (i32, i32))> = HashSet::new();
    let (mut guard_position, mut dir) = (find_guard(map), (0, -1));
    loop {
        if hash_set.contains(&((guard_position.0, guard_position.1, dir))) {
            return true;
        };
        hash_set.insert((guard_position.0, guard_position.1, dir));
        let position_in_front = (
            guard_position.0 as i32 + dir.0,
            guard_position.1 as i32 + dir.1,
        );

        if !is_valid(
            position_in_front.0,
            position_in_front.1,
            map[0].len(),
            map[1].len(),
        ) {
            break;
        }

        if map[position_in_front.1 as usize][position_in_front.0 as usize] == '#' {
            dir = successon_dir(dir);
            continue;
        }

        guard_position = (
            (guard_position.0 as i32 + dir.0) as usize,
            (guard_position.1 as i32 + dir.1) as usize,
        );
    }
    false
}

pub fn part_two() -> i32 {
    let mut possible_locations = 0;
    let mut map = get_map();
    let (mut guard_position, mut dir) = (find_guard(&map), (0, -1));
    loop {
        if map[guard_position.1][guard_position.0] == '.' {
            map[guard_position.1][guard_position.0] = 'X';
        }

        let position_in_front = (
            guard_position.0 as i32 + dir.0,
            guard_position.1 as i32 + dir.1,
        );

        if !is_valid(
            position_in_front.0,
            position_in_front.1,
            map[0].len(),
            map[1].len(),
        ) {
            break;
        }

        if map[position_in_front.1 as usize][position_in_front.0 as usize] == '.' {
            let mut candidate_map = map.clone();
            candidate_map[position_in_front.1 as usize][position_in_front.0 as usize] = '#';
            if creates_loop(&candidate_map) {
                possible_locations += 1;
            }
        }

        if map[position_in_front.1 as usize][position_in_front.0 as usize] == '#' {
            dir = successon_dir(dir);
            continue;
        }

        guard_position = (
            (guard_position.0 as i32 + dir.0) as usize,
            (guard_position.1 as i32 + dir.1) as usize,
        );
    }
    possible_locations
}
