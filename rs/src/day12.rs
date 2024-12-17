use std::collections::HashSet;

const INPUT: &str = include_str!("./input12.txt");

fn get_data() -> Vec<Vec<char>> {
    INPUT.lines().map(|x| x.chars().collect()).collect()
}

fn is_valid(x: i32, y: i32, lenx: usize, leny: usize) -> bool {
    x >= 0 && y >= 0 && x < lenx as i32 && y < leny as i32
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn bfs(
    border: &HashSet<(usize, usize)>,
    data: &Vec<Vec<char>>,
    current_region: &HashSet<(usize, usize)>,
    l: char,
) -> HashSet<(usize, usize)> {
    let mut new_border = HashSet::new();
    for b in border {
        for d in DIRECTIONS {
            let n = (b.0 as i32 + d.0, b.1 as i32 + d.1);
            if !is_valid(n.0, n.1, data.len(), data[0].len()) {
                continue;
            }
            if data[n.0 as usize][n.1 as usize] == l
                && !current_region.contains(&(n.0 as usize, n.1 as usize))
            {
                new_border.insert((n.0 as usize, n.1 as usize));
            }
        }
    }
    new_border
}

fn get_regions(data: &Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let mut regions = Vec::new();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut current_region: HashSet<(usize, usize)> = HashSet::new();

    for row in 0..data.len() {
        for column in 0..data[0].len() {
            if seen.contains(&(row, column)) {
                continue;
            }
            let l = data[row][column];
            let mut current_border = vec![(row, column)].into_iter().collect();
            current_region.insert((row, column));

            while !bfs(&current_border, data, &current_region, l).is_empty() {
                current_border = bfs(&current_border, data, &current_region, l);
                for x in &current_border {
                    current_region.insert(*x);
                }
            }

            let v = current_region
                .iter()
                .map(|x| x.clone())
                .collect::<Vec<(usize, usize)>>();
            for x in &v {
                seen.insert(*x);
            }
            regions.push(v);
            current_region = HashSet::new();
        }
    }

    regions
}

fn area(r: &Vec<(usize, usize)>) -> i32 {
    r.len() as i32
}

fn perimeter(r: &Vec<(usize, usize)>, grid: &Vec<Vec<char>>) -> i32 {
    let l = grid[r[0].0][r[0].1];
    let mut count = 0;

    for p in r {
        for d in DIRECTIONS {
            let n = ((p.0 as i32 + d.0), (p.1 as i32 + d.1));
            if !is_valid(n.0, n.1, grid.len(), grid[0].len())
                || grid[n.0 as usize][n.1 as usize] != l
            {
                count += 1;
            }
        }
    }

    count
}

//pad out grid with extra row, column
//for each region: create the set of all square adjacent to the region which DOES double count
//squares which are adjacent to more than one side
//number of sides = length of this set - number of adjacencies in this set

fn adjacencies(r: &Vec<(usize, usize)>, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut h = HashSet::new();
    let l = grid[r[0].0][r[0].1];

    for x in r {
        for d in DIRECTIONS {
            let n = (x.0 as i32 + d.0, x.1 as i32 + d.1);
            if grid[n.0 as usize][n.1 as usize] != l {
                h.insert((n.0 as usize, n.1 as usize));
            }
        }
    }
    h.into_iter().collect()
}

fn is_adjacent(a: (usize, usize), b: (usize, usize)) -> bool {
    for d in DIRECTIONS {
        let n = (a.0 as i32 + d.0, a.1 as i32 + d.1);
        if b.0 as i32 == n.0 && b.1 as i32 == n.1 {
            return true;
        }
    }
    false
}

fn sides(r: &Vec<(usize, usize)>, grid: &Vec<Vec<char>>) -> i32 {
    let l = grid[r[0].0][r[0].1];
    let (mut padded, mut n) = (Vec::new(), Vec::new());
    for _ in 0..grid[0].len() + 2 {
        n.push(' ');
    }
    padded.push(n.clone());
    for r in grid {
        padded.push(vec![vec![vec![' '], r.to_vec()].concat(), vec![' ']].concat());
    }
    padded.push(n);

    let fixed_indices = r.iter().map(|x| (x.0 + 1, x.1 + 1)).collect();
    let adj = adjacencies(&fixed_indices, &padded);
    let mut sides = 0;

    for a in adj {
        for d in DIRECTIONS {
            let n = (a.0 as i32 + d.0, a.1 as i32 + d.1);
            if !is_valid(n.0, n.1, padded.len(), padded[0].len()) {
                continue;
            }
            if fixed_indices.contains(&(n.0 as usize, n.1 as usize)) {
                sides += 1;
            }
        }
    }

    let mut coincidences = 0;
    for a in &fixed_indices {
        let mut cs = Vec::new();
        for b in &fixed_indices {
            if is_adjacent(*a, *b) {
                cs.push(*b);
            }
        }

        for c in cs {
            for d in DIRECTIONS {
                let (n1, n2) = (
                    (a.0 as i32 + d.0, a.1 as i32 + d.1),
                    (c.0 as i32 + d.0, c.1 as i32 + d.1),
                );
                if !(is_valid(n1.0, n1.1, padded.len(), padded[0].len())
                    && is_valid(n2.0, n2.1, padded.len(), padded[0].len()))
                {
                    continue;
                }
                if padded[n1.0 as usize][n1.1 as usize] != l
                    && padded[n2.0 as usize][n2.1 as usize] != l
                {
                    coincidences += 1;
                }
            }
        }
    }
    sides -= coincidences / 2;
    sides
}

pub fn part_one() -> i32 {
    let data = get_data();
    let regions = get_regions(&data);

    regions
        .iter()
        .fold(0, |acc, x| acc + area(x) * perimeter(x, &data))
}

pub fn part_two() -> i32 {
    let data = get_data();
    let regions = get_regions(&data);

    regions
        .iter()
        .fold(0, |acc, x| acc + area(x) * sides(x, &data))
}
