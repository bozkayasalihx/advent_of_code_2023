use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("coullnd't opne the file");
    let lines = input.lines().collect::<Vec<_>>();

    let res = part_one(&lines);

    println!("{}", res);
}
fn part_one(lines: &[&str]) -> u32 {
    let mut total = 0;
    let two: u32 = 2;
    for line in lines {
        let (taken_side, win_side) = line.split_once(" | ").unwrap();
        let taken_side = taken_side
            .split_once(": ")
            .unwrap()
            .1
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        let win_side = win_side
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let mut count: u32 = 0;

        for r in win_side {
            for l in &taken_side {
                if &r == l {
                    count += 1;
                }
            }
        }

        if count > 0 {
            total += two.pow(count - 1);
        }
        count = 0;
    }
    total
}
