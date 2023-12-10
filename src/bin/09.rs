use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut final_sum: i64 = 0;
    for line in input.lines() {
        let mut patterns: Vec<Vec<i64>> = Vec::new();
        patterns.push(line.split(' ').map(|a| a.parse::<i64>().unwrap()).collect());

        while patterns.last().unwrap().iter().any(|v| *v != 0) {
            patterns.push(
                patterns
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<_>>(),
            );
        }
        patterns.last_mut().unwrap().push(0);

        for i in (0..patterns.len() - 1).rev() {
            let sum = patterns[i].last().unwrap() + patterns[i + 1].last().unwrap();
            patterns.get_mut(i).unwrap().push(sum);
        }

        final_sum += *patterns.first()?.last().unwrap();
    }

    Some(final_sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut final_sum: i64 = 0;
    for line in input.lines() {
        let mut patterns: Vec<VecDeque<i64>> = Vec::new();
        patterns.push(line.split(' ').map(|a| a.parse::<i64>().unwrap()).collect());

        while patterns.last().unwrap().iter().any(|v| *v != 0) {
            patterns.push(
                patterns
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows::<(_, _)>()
                    .map(|window| window.1 - window.0)
                    .collect::<VecDeque<_>>(),
            );
        }
        patterns.last_mut().unwrap().push_back(0);

        for i in (0..patterns.len() - 1).rev() {
            let sum = patterns[i].front().unwrap() - patterns[i + 1].front().unwrap();
            patterns.get_mut(i).unwrap().push_front(sum);
        }

        final_sum += *patterns.first()?.front().unwrap();
    }

    Some(final_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2));
    }
}
