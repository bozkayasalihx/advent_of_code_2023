use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read the file");
    let lines = input.lines().collect::<Vec<_>>();
    // let ans = part_one(&lines);
    let ans = part_two(&lines);
    println!("{}", ans);
}

fn part_one(lines: &[&str]) -> u32 {
    let time = lines[0]
        .split_once("Time: ")
        .expect("couldnt' split the time value")
        .1
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let dest = lines[1]
        .split_once("Distance: ")
        .expect("couldn't split the distance value")
        .1
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut total = 1;
    for (idx, t) in time.iter().enumerate() {
        let dist = dest[idx];
        let mut cur = 0;
        for i in 1..=*t {
            if (t - i) * i > dist {
                println!("t {} -> i {}", t, i);
                cur += 1;
            }
        }
        total *= cur;
        cur = 0;
    }

    total
}

fn part_two(lines: &[&str]) -> usize {
    let time = lines[0]
        .split_once("Time: ")
        .expect("couldnt' split the time value")
        .1
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let dest = lines[1]
        .split_once("Distance: ")
        .expect("couldn't split the distance value")
        .1
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    println!("dest {}", dest);
    println!("time {}", time);
    let mut cur = 0;
    for i in 1..=time {
        if (time - i) * i > dest {
            cur += 1;
        }
    }
    cur
}
