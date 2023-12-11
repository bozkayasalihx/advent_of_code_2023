use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't open input.txt file");
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let empty_rows = matrix
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| {
            if line.iter().all(|&x| x == '.') {
                return Some(idx);
            }
            None
        })
        .collect::<Vec<_>>();

    // let empty_cols = (0..matrix[0].len())
    //     .filter(|&col| matrix.iter().all(|row| row[col] == '.'))
    //     .flat_map(|col| matrix.iter().map(|row| row[col]).collect::<Vec<_>>())
    //     .collect::<Vec<_>>();
    let empty_cols = (0..matrix[0].len())
        .filter(|&col| matrix.iter().all(|row| row[col] == '.'))
        .collect::<Vec<_>>();

    let points = matrix
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, c)| {
                    if *c == '#' {
                        return Some((row, col));
                    }
                    None
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans = part_one(&points, &empty_cols, &empty_rows);
    // let ans = part_two(&points, &empty_cols, &empty_rows);
    println!("{}", ans);
}

fn part_one(points: &Vec<(usize, usize)>, empty_cols: &Vec<usize>, empty_rows: &Vec<usize>) -> u64 {
    let mut total = 0;
    for (idx, &(r1, c1)) in points.iter().enumerate() {
        for &(r2, c2) in points[idx..].iter() {
            for r in cmp::min(r1, r2)..cmp::max(r1, r2) {
                if empty_rows.contains(&r) {
                    total += 2;
                } else {
                    total += 1;
                }
            }

            for c in cmp::min(c1, c2)..cmp::max(c1, c2) {
                if empty_cols.contains(&c) {
                    total += 2;
                } else {
                    total += 1;
                }
            }
        }
    }

    total
}

fn part_two(points: &Vec<(usize, usize)>, empty_cols: &Vec<usize>, empty_rows: &Vec<usize>) -> u64 {
    let mut total = 0;
    for (idx, &(r1, c1)) in points.iter().enumerate() {
        for &(r2, c2) in points[idx..].iter() {
            for r in cmp::min(r1, r2)..cmp::max(r1, r2) {
                if empty_rows.contains(&r) {
                    total += 1000000;
                } else {
                    total += 1;
                }
            }

            for c in cmp::min(c1, c2)..cmp::max(c1, c2) {
                if empty_cols.contains(&c) {
                    total += 1000000;
                } else {
                    total += 1;
                }
            }
        }
    }

    total
}
