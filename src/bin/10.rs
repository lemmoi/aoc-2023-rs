use std::{collections::VecDeque, slice::Iter};

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy)]
enum Pipe {
    Horizontal,
    Vertical,
    BendL,
    BendJ,
    Bend7,
    BendF,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
    entered_from: Direction,
}

type Grid = Vec<Vec<char>>;

impl Pipe {
    fn from_char(c: char) -> Option<Pipe> {
        match c {
            '-' => Some(Pipe::Horizontal),
            '|' => Some(Pipe::Vertical),
            'L' => Some(Pipe::BendL),
            'J' => Some(Pipe::BendJ),
            '7' => Some(Pipe::Bend7),
            'F' => Some(Pipe::BendF),
            _ => None,
        }
    }

    fn next_direction(self, entering_from: Direction) -> Option<Direction> {
        match self {
            Pipe::Horizontal => match entering_from {
                Direction::Left => Some(Direction::Right),
                Direction::Right => Some(Direction::Left),
                _ => None,
            },
            Pipe::Vertical => match entering_from {
                Direction::Up => Some(Direction::Down),
                Direction::Down => Some(Direction::Up),
                _ => None,
            },
            Pipe::BendL => match entering_from {
                Direction::Up => Some(Direction::Right),
                Direction::Right => Some(Direction::Up),
                _ => None,
            },
            Pipe::BendJ => match entering_from {
                Direction::Left => Some(Direction::Up),
                Direction::Up => Some(Direction::Left),
                _ => None,
            },
            Pipe::Bend7 => match entering_from {
                Direction::Left => Some(Direction::Down),
                Direction::Down => Some(Direction::Left),
                _ => None,
            },
            Pipe::BendF => match entering_from {
                Direction::Down => Some(Direction::Right),
                Direction::Right => Some(Direction::Down),
                _ => None,
            },
        }
    }
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        DIRECTIONS.iter()
    }

    pub fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Position {
    pub fn go_towards(&self, dir: &Direction) -> Option<Position> {
        let (row, col) = match dir {
            Direction::Up => (self.row as i32 - 1, self.col as i32),
            Direction::Down => (self.row as i32 + 1, self.col as i32),
            Direction::Left => (self.row as i32, self.col as i32 - 1),
            Direction::Right => (self.row as i32, self.col as i32 + 1),
        };
        if row < 0 || col < 0 {
            None
        } else {
            Some(Position {
                row: row as usize,
                col: col as usize,
                entered_from: dir.inverse(),
            })
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Mark {
    NotVisited,
    Visited(u32, Pipe),
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let distances = bfs(&grid);
    distances
        .iter()
        .filter_map(|row| {
            row.iter()
                .filter_map(|mark| {
                    if let Mark::Visited(dist, _) = mark {
                        Some(*dist)
                    } else {
                        None
                    }
                })
                .max()
        })
        .max()
}

fn bfs(grid: &Grid) -> Vec<Vec<Mark>> {
    let mut distances: Vec<Vec<Mark>> = vec![vec![Mark::NotVisited; grid[0].len()]; grid.len()];
    let start = find_start(grid);
    distances[start.0][start.1] = Mark::Visited(0, Pipe::Horizontal);

    let mut queue = VecDeque::new();
    for dir in Direction::iterator() {
        if let Some(next_pos) = (Position {
            row: start.0,
            col: start.1,
            entered_from: Direction::Down,
        })
        .go_towards(dir)
        {
            queue.push_back((next_pos, 1));
        }
    }

    while !queue.is_empty() {
        let (cur_pos, distance_from_start) = queue.pop_front().unwrap();
        let cur_char = grid.get(cur_pos.row).and_then(|row| row.get(cur_pos.col));

        if cur_char.is_none() {
            // outside of bounds
            continue;
        }

        if let Mark::Visited(_, _) = distances[cur_pos.row][cur_pos.col] {
            // already visitied
            continue;
        }

        if let Some(pipe) = Pipe::from_char(*cur_char.unwrap()) {
            if let Some(next_dir) = pipe.next_direction(cur_pos.entered_from) {
                distances[cur_pos.row][cur_pos.col] = Mark::Visited(distance_from_start, pipe);
                if let Some(next_pos) = cur_pos.go_towards(&next_dir) {
                    queue.push_back((next_pos, distance_from_start + 1));
                }
            }
        }
    }

    // Set start to a vertical pipe if necessary for poly fill
    if 1 <= start.0 && start.0 < distances.len() - 1 {
        if let Mark::Visited(_, pipe) = distances[start.0 - 1][start.1] {
            match pipe {
                Pipe::Vertical | Pipe::Bend7 | Pipe::BendF => {
                    println!("Setting to vertical");
                    distances[start.0][start.1] = Mark::Visited(0, Pipe::Vertical);
                }
                _ => (),
            }
        }
    }

    distances
}

fn find_start(grid: &Grid) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .position(|c| *c == 'S')
                .map(|col_idx| (row_idx, col_idx))
        })
        .unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let distances = bfs(&grid);

    let total_enclosed: u32 = distances
        .iter()
        .map(|row| {
            let mut sum = 0;
            let mut in_poly = false;
            for mark in row {
                if let Mark::Visited(_, pipe) = mark {
                    let is_changed = match pipe {
                        Pipe::Horizontal => false,
                        Pipe::Vertical => true,
                        Pipe::BendL => true,
                        Pipe::BendJ => true,
                        Pipe::Bend7 => false,
                        Pipe::BendF => false,
                    };
                    if is_changed {
                        in_poly = !in_poly;
                    }
                } else if in_poly {
                    sum += 1;
                }
            }
            sum
        })
        .sum();

    Some(total_enclosed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(10));
    }
}
