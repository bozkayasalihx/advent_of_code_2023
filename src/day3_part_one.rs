use std::fs;

#[derive(Debug, Default)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Default)]
struct Num {
    str_content: String,
    len: usize,
    coord: Coord,
    symbols: Vec<char>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("coulnd't open the file");
    let lines = input.lines().collect::<Vec<_>>();

    part_one(&lines);
}

fn part_one(lines: &[&str]) -> u32 {
    let mut appending_num = false;
    let mut num = Num::default();
    let mut total = Vec::new();

    let row_len = lines[0].len();
    let col_len = lines.len();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_digit(10) && !appending_num {
                continue;
            }

            if c.is_digit(10) {
                if !appending_num {
                    num = Num {
                        str_content: c.to_string(),
                        len: 1,
                        coord: Coord { x, y },
                        symbols: Vec::new(),
                    };
                    appending_num = true;
                } else {
                    num.str_content += &c.to_string();
                    num.len += 1;
                }
            }

            if appending_num && (!c.is_digit(10) || x == row_len - 1) {
                let min_x = if num.coord.x == 0 { 0 } else { num.coord.x - 1 };
                let max_x = if num.coord.x + num.len < row_len {
                    num.coord.x + num.len
                } else {
                    row_len - 1
                };

                let min_y = if num.coord.y > 0 { y - 1 } else { y };
                let max_y = if num.coord.y < col_len - 1 { y + 1 } else { y };

                for check_y in min_y..=max_y {
                    for check_char in lines[check_y][min_x..=max_x].chars() {
                        if !check_char.is_digit(10) && check_char != '.' {
                            num.symbols.push(check_char);
                        }
                    }
                }

                appending_num = false;
                total.push(num);
            }
        }
    }

    total.iter().sum()
}
