use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't open the file");
    let lines = input.lines().collect::<Vec<_>>();
    let ans = part_one(&lines);

    println!("{}", ans);
}

fn part_one(lines: &[&str]) -> i32 {
    let mut s = Vec::<i32>::new();
    for line in lines {
        let mut t = 0;
        let line = line
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        t += find_diff(&line);
        t += line.last().unwrap();
        s.push(t);
    }

    s.iter().sum()
}

fn find_diff(line: &Vec<i32>) -> i32 {
    let mut t = Vec::<i32>::new();
    let mut iter = line.iter();
    let mut c = iter.next().unwrap_or(&0);

    while let Some(n) = iter.next() {
        t.push(n - c);
        c = n;
    }

    if t.iter().all(|x| *x == 0) {
        return 0;
    }

    return t.last().unwrap() + find_diff(&t);
}
