use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Clone)]
struct Pipe {
    dirs: Vec<Dir>,
    coord: (usize, usize),
    dest: u32,
    sym: char,
}

impl Pipe {
    fn can_move(&self, dir: &Dir) -> Option<Dir> {
        match dir {
            Dir::N => {
                if self.dirs.contains(&Dir::S) {
                    return Some(Dir::S);
                }

                None
            }
            Dir::S => {
                if self.dirs.contains(&Dir::N) {
                    return Some(Dir::N);
                }
                None
            }
            Dir::W => {
                if self.dirs.contains(&Dir::E) {
                    return Some(Dir::E);
                }
                None
            }

            Dir::E => {
                if self.dirs.contains(&Dir::W) {
                    return Some(Dir::W);
                }
                None
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read input.txt");

    let ans = part_one(&input);
    println!("{}", ans);
}

fn part_one(input: &str) -> u32 {
    let mut start_point = (0, 0);
    let matrix: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, col)| match col {
                    '|' => Pipe {
                        dirs: vec![Dir::N, Dir::S],
                        dest: 0,
                        coord: (row_idx, col_idx),
                        sym: col,
                    },
                    '-' => Pipe {
                        dirs: vec![Dir::W, Dir::E],
                        dest: 0,
                        coord: (row_idx, col_idx),
                        sym: col,
                    },
                    'L' => Pipe {
                        dirs: vec![Dir::N, Dir::E],
                        dest: 0,
                        coord: (row_idx, col_idx),
                        sym: col,
                    },

                    'J' => Pipe {
                        dirs: vec![Dir::N, Dir::W],
                        dest: 0,
                        coord: (row_idx, col_idx),
                        sym: col,
                    },

                    'F' => Pipe {
                        dirs: vec![Dir::S, Dir::E],
                        dest: 0,
                        coord: (row_idx, col_idx),
                        sym: col,
                    },

                    '7' => Pipe {
                        dirs: vec![Dir::S, Dir::W],
                        dest: 0,
                        coord: (row_idx, col_idx),
                        sym: col,
                    },

                    '.' => Pipe {
                        dirs: vec![],
                        dest: 0,
                        coord: (row_idx, col_idx),
                        sym: col,
                    },

                    'S' => {
                        start_point = (row_idx, col_idx);
                        Pipe {
                            dirs: vec![Dir::W, Dir::S, Dir::E, Dir::N],
                            dest: 0,
                            coord: (row_idx, col_idx),
                            sym: col,
                        }
                    }
                    _ => panic!("invalid char"),
                })
                .collect()
        })
        .collect();

    let matrix_size = (matrix.len(), matrix[0].len());
    let mut visited = HashMap::new();
    let start_pipe = matrix[start_point.0][start_point.1].clone();
    bfs(&matrix, &matrix_size, &mut visited, start_pipe);

    visited.into_values().max().unwrap_or(0)
}

fn bfs(
    matrix: &Vec<Vec<Pipe>>,
    matrix_size: &(usize, usize),
    visited: &mut HashMap<(usize, usize), u32>,
    visitor: Pipe,
) {
    let mut queue = VecDeque::new();
    queue.push_front(visitor);

    while !queue.is_empty() {
        let visitor = queue.pop_front().unwrap();
        visited.insert(visitor.coord, visitor.dest);
        for dir in &visitor.dirs {
            let (dx, dy) = match dir {
                Dir::N => (-1, 0),
                Dir::S => (1, 0),
                Dir::W => (0, -1),
                Dir::E => (0, 1),
            };

            let x = (visitor.coord.0 as i32 + dx) as usize;
            let y = (visitor.coord.1 as i32 + dy) as usize;

            if x >= matrix_size.0 || y >= matrix_size.1 || visited.contains_key(&(x, y)) {
                continue;
            }

            let mut potential_visitor = matrix[x][y].clone();

            if let Some(backward) = potential_visitor.can_move(&dir) {
                let backward_idx = potential_visitor
                    .dirs
                    .iter()
                    .position(|x| *x == backward)
                    .unwrap();

                potential_visitor.dirs.swap_remove(backward_idx);
                potential_visitor.dest += visitor.dest + 1;
                queue.push_back(potential_visitor.clone());
            }
        }
    }
}
