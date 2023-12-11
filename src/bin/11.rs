use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Clone, Copy, Debug)]
enum Space {
    Empty,
    Galaxy,
}

#[derive(Clone, Copy, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn manhattan(&self, other: &Position) -> u32 {
        (self.row as i32).abs_diff(other.row as i32) + (self.col as i32).abs_diff(other.col as i32)
    }
}

type Grid<T> = Vec<Vec<T>>;

impl Space {
    fn from_char(c: char) -> Space {
        match c {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => panic!("Unknown char"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<Space> = input
        .lines()
        .map(|line| line.chars().map(Space::from_char).collect())
        .collect();

    let galaxies = expand_space(grid, 2);
    Some(
        galaxies
            .iter()
            .combinations(2)
            .map(|neighbors| neighbors[0].manhattan(neighbors[1]))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Grid<Space> = input
        .lines()
        .map(|line| line.chars().map(Space::from_char).collect())
        .collect();
    let galaxies = expand_space(grid, 1_000_000);
    Some(
        galaxies
            .iter()
            .combinations(2)
            .map(|neighbors| neighbors[0].manhattan(neighbors[1]) as u64)
            .sum(),
    )
}

fn expand_space(grid: Grid<Space>, ratio: u32) -> Vec<Position> {
    let empty_rows: Vec<_> = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if row.iter().all(|s| matches!(s, Space::Empty)) {
                Some(idx)
            } else {
                None
            }
        })
        .collect();

    let empty_cols = grid
        .iter()
        .map(|row| {
            // For each row, vec of columns that have empty spaces
            row.iter()
                .enumerate()
                .filter_map(|(col_idx, col)| {
                    if matches!(col, Space::Empty) {
                        Some(col_idx)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .fold((0..grid[0].len()).collect::<Vec<_>>(), |mut acc, i| {
            acc.retain(|col| i.contains(col));
            acc
        });

    let mut galaxies = Vec::new();
    let mut added_rows = 0;
    let expansion: usize = ratio as usize - 1;
    for i in 0..grid.len() {
        if added_rows < empty_rows.len() && empty_rows[added_rows] < i {
            added_rows = empty_rows.iter().filter(|idx| **idx < i).count();
        }
        let mut added_cols = 0;
        for j in 0..grid[0].len() {
            if matches!(grid[i][j], Space::Galaxy) {
                if added_cols < empty_cols.len() && empty_cols[added_cols] < j {
                    added_cols = empty_cols.iter().filter(|idx| **idx < j).count();
                }

                galaxies.push(Position {
                    row: i + added_rows * expansion,
                    col: j + added_cols * expansion,
                });
            }
        }
    }
    galaxies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
