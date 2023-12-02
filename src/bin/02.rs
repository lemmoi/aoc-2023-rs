advent_of_code::solution!(2);

use regex::Regex;
use std::cmp;



pub fn part_one(input: &str) -> Option<u32> {
    run(input, |i, r, g, b| {
        if r <= 12 && g <= 13 && b <= 14 {
            Some(i + 1)
        } else {
            None
        }
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    run(input, |_, r, g, b| {Some(r * g * b)})
}

fn run<F>(input: &str, f: F) -> Option<u32> where F: Fn(u32, u32, u32, u32) -> Option<u32> {
    let re = Regex::new(r"(\d+) (b|r|g)").unwrap();
    Some(input.lines().enumerate().map(|(game, line)| {
            let rounds = line[5..].split(';');
            let mut r: u32 = 0;
            let mut b: u32 = 0;
            let mut g: u32 = 0;
            for round in rounds {
                re.captures_iter(round).for_each(|entry| { 
                    let (_, [count, letter]) = entry.extract();
                    let num: u32 = count.parse().unwrap();
                    match letter {
                        "r" => r = cmp::max(r, num),
                        "b" => b = cmp::max(b, num),
                        "g" => g = cmp::max(g, num),
                        _ => panic!("frick")
                    }
                });
            }
            f(game as u32, r, g, b)
        }).filter_map(|x| x)
        .sum())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
