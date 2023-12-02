use std::fs;

#[derive(Debug, Default)]
struct Turn {
    red: u32,
    green: u32,
    blue: u32,
}

impl Turn {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn main() {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes?
    let input = fs::read_to_string("input.txt").expect("couldn't read the file");
    let lines = input.lines().collect::<Vec<_>>();
    // let total = part_one(&lines).expect("couldn't get idxs");
    let total = part_two(&lines).expect("couldn't get idxs");
    println!("{:?}", total);
}

fn part_one(lines: &[&str]) -> Option<usize> {
    let mut total = 0;
    'next: for line in lines {
        let (g, turns) = line.split_once(": ")?;
        let g = g.split_once(' ')?.1.parse::<usize>().unwrap();
        total += g;
        let turns = turns.split("; ").collect::<Vec<_>>();
        for t in turns {
            let cubes = t.split(", ").collect::<Vec<_>>();
            let mut turn = Turn::default();
            for c in cubes {
                let (amount, color) = c.split_once(' ')?;
                let amount = amount
                    .parse::<u32>()
                    .expect("couldn't parse the turn amount");
                match color {
                    "red" => turn.red = amount,
                    "green" => turn.green = amount,
                    "blue" => turn.blue = amount,
                    _ => {}
                }
            }
            if !turn.is_valid() {
                total -= g;
                continue 'next;
            }
        }
    }

    Some(total)
}
fn part_two(lines: &[&str]) -> Option<u32> {
    let mut games = Vec::new();
    for line in lines {
        let (g, turns) = line.split_once(": ")?;
        let g = g.split_once(' ')?.1.parse::<usize>().unwrap();
        let turns = turns.split("; ").collect::<Vec<_>>();
        let mut turn_list = Vec::new();
        for t in turns {
            let cubes = t.split(", ").collect::<Vec<_>>();
            let mut turn = Turn::default();
            for c in cubes {
                let (amount, color) = c.split_once(' ')?;
                let amount = amount
                    .parse::<u32>()
                    .expect("couldn't parse the turn amount");
                match color {
                    "red" => turn.red = amount,
                    "green" => turn.green = amount,
                    "blue" => turn.blue = amount,
                    _ => {}
                }
            }
            turn_list.push(turn);
        }

        games.push(turn_list);
    }
    let mut total = 0;
    for turns in games {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for turn in turns {
            if turn.red > red {
                red = turn.red;
            }

            if turn.blue > blue {
                blue = turn.blue;
            }

            if turn.green > green {
                green = turn.green;
            }
        }
        total += red * green * blue;
    }

    Some(total)
}
