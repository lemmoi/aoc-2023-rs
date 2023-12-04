advent_of_code::solution!(3);

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Number {
    val: u32,
    start: Point,
    end: Point,
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    val: char,
    point: Point,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, symbols): (Vec<Number>, Vec<Symbol>) = parse_grid(input);
    Some(
        numbers
            .into_iter()
            .filter(|num| {
                symbols.iter().map(|symbol| symbol.point).any(|point| {
                    point.x >= num.start.x - 1
                        && point.x <= num.end.x + 1
                        && point.y >= num.start.y - 1
                        && point.y <= num.start.y + 1
                })
            })
            .map(|num| num.val)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (numbers, symbols): (Vec<Number>, Vec<Symbol>) = parse_grid(input);
    Some(
        symbols
            .iter()
            .filter_map(|symbol| {
                if symbol.val == '*' {
                    Some(symbol.point)
                } else {
                    None
                }
            })
            .filter_map(|point| {
                let nums = numbers
                    .iter()
                    .filter(|num| {
                        point.x >= num.start.x - 1
                            && point.x <= num.end.x + 1
                            && point.y >= num.start.y - 1
                            && point.y <= num.start.y + 1
                    })
                    .collect::<Vec<_>>();

                if nums.len() == 2 {
                    Some((*nums.get(0).unwrap(), *nums.get(1).unwrap()))
                } else {
                    None
                }
            })
            .map(|nums| nums.0.val * nums.1.val)
            .sum(),
    )
}

fn parse_grid(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        let mut cur_number: Option<(u32, Point)> = None;
        for (col, c) in line.iter().enumerate() {
            match c.to_digit(10) {
                Some(digit) => {
                    cur_number = match cur_number {
                        Some((partial_num, start)) => Some((partial_num * 10 + digit, start)),
                        None => Some((
                            digit,
                            Point {
                                x: col as i32,
                                y: row as i32,
                            },
                        )),
                    }
                }
                None => {
                    if let Some((partial_num, start)) = cur_number {
                        numbers.push(Number {
                            val: partial_num,
                            start,
                            end: Point {
                                x: col as i32 - 1,
                                y: row as i32,
                            },
                        });
                        cur_number = None;
                    };
                    if *c != '.' {
                        symbols.push(Symbol {
                            val: *c,
                            point: Point {
                                x: col as i32,
                                y: row as i32,
                            },
                        })
                    }
                }
            }
        }
        if let Some((cur_num, start)) = cur_number {
            numbers.push(Number {
                val: cur_num,
                start,
                end: Point {
                    x: line.len() as i32 - 1,
                    y: row as i32,
                },
            });
        };
    }
    (numbers, symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
