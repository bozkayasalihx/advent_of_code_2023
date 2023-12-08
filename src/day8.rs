use std::collections::HashMap;
use std::fs;

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

    let ans = part_one(&map_hash, insts);
    println!("{ans}");
}

fn part_one(hash_data: &HashMap<&str, (&str, &str)>, insts: Vec<char>) -> u32 {
    let mut counter = 0;
    let mut current = "AAA";

    'looper: loop {
        for inst in &insts {
            match inst {
                'R' => {
                    counter += 1;
                    current = &hash_data.get(current).unwrap().1;

                    if current == "ZZZ" {
                        break 'looper;
                    }
                }
                'L' => {
                    counter += 1;
                    current = &hash_data.get(current).unwrap().0;

                    if current == "ZZZ" {
                        break 'looper;
                    }
                }
                _ => panic!("shouldn't be reached"),
            }
        }
    }

    counter
}
