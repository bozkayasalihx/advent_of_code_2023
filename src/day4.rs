use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("coullnd't opne the file");
    let lines = input.lines().collect::<Vec<_>>();

    let res = part_one(&lines);
    let res = part_two(&lines);

    println!("{}", res);
}
fn part_one(lines: &[&str]) -> u32 {
    let mut total = 0;
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
                    break;
                }
            }
        }

        if count > 0 {
            total += u32::pow(2, count - 1);
        }
        count = 0;
    }
    total
}

fn part_two(lines: &[&str]) -> u32 {
    let mut cards = lines.iter().map(|_| 1).collect::<Vec<_>>();

    for (idx, line) in lines.iter().enumerate() {
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
        let mut count: usize = 0;

        for r in win_side {
            for l in &taken_side {
                if &r == l {
                    count += 1;
                    break;
                }
            }
        }

        let [amount, next_cards @ ..] = &mut cards[idx..=(idx + count)] else {
            unreachable!();
        };
        next_cards.iter_mut().for_each(|c| *c += *amount);
    }

    cards.into_iter().sum()
}
