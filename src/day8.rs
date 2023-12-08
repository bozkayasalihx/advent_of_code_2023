use std::collections::HashMap;
use std::{fs, usize};

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't open the file");
    let (inst, maps) = input
        .split_once("\n\n")
        .expect("couldnt get instruction from maps");

    let mut map_hash = HashMap::<&str, (&str, &str)>::new();
    let insts = inst.chars().collect::<Vec<_>>();

    for m in maps.lines() {
        let (lhs, m) = m.split_once(" = ").expect("coudn't split the data");
        let (x, y) = m
            .split_once("(")
            .unwrap()
            .1
            .split_once(")")
            .unwrap()
            .0
            .split_once(", ")
            .unwrap();

        map_hash.insert(lhs, (x, y));
    }

    // let ans = part_one(&map_hash, insts);
    let ans = part_two(&map_hash, insts);
    println!("{ans}");
}

fn part_one(hash_data: &HashMap<&str, (&str, &str)>, insts: Vec<char>) -> usize {
    let current = "AAA";
    count_loop(hash_data, &insts, current)
}

fn part_two(hash_data: &HashMap<&str, (&str, &str)>, insts: Vec<char>) -> usize {
    let count = hash_data
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| count_loop(hash_data, &insts, x))
        .reduce(|acc, cur| lcm(acc, cur))
        .unwrap();

    count
}

fn count_loop<'a>(
    hash_data: &HashMap<&'a str, (&'a str, &'a str)>,
    insts: &Vec<char>,
    mut current: &'a str,
) -> usize {
    let mut counter = 0;
    'looper: loop {
        for inst in insts {
            match inst {
                'R' => {
                    counter += 1;
                    current = hash_data.get(current).unwrap().1;

                    if current.chars().last() == Some('Z') {
                        break 'looper;
                    }
                }
                'L' => {
                    counter += 1;
                    current = &hash_data.get(current).unwrap().0;

                    if current.chars().last() == Some('Z') {
                        break 'looper;
                    }
                }
                _ => panic!("shouldn't be reached"),
            }
        }
    }

    counter
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
