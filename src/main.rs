use std::fs;

#[derive(Debug, Default, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Clone)]
struct Sym {
    sym: char,
    coord: Coord,
}

#[derive(Debug, Default, Clone)]
struct Num {
    str_content: String,
    len: usize,
    coord: Coord,
    symbols: Vec<Sym>,
}

#[derive(Debug, Default)]
struct Gear {
    sym: Sym,
    nums: Vec<u32>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("coulnd't open the file");
    let lines = input.lines().collect::<Vec<_>>();

    let res = part_two(&lines);
    println!("{}", res);
}

fn part_two(lines: &[&str]) -> u32 {
    let mut appending_num = false;
    let mut num = Num::default();
    let mut total = Vec::<Num>::new();

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
                    for (check_x, check_char) in lines[check_y][min_x..=max_x].chars().enumerate() {
                        if !check_char.is_digit(10) && check_char != '.' {
                            num.symbols.push(Sym {
                                sym: check_char,
                                coord: Coord {
                                    x: check_x,
                                    y: check_y,
                                },
                            });
                        }
                    }
                }

                appending_num = false;
                total.push(num.clone());
            }
        }
    }

    for num in total {
        if num.symbols.len() > 0 && num.symbols[0].sym == '*' {
            let b_x = num.coord.x;
            let e_x = num.coord.x + num.len;
            let b_y = num.coord.y;

            let sym_coord = num.symbols[0].coord;
        }
    }
    0
}
