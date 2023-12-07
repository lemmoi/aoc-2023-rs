advent_of_code::solution!(6);

use std::iter::zip;

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let re = Regex::new(r"(\d+)").unwrap();
    let times: Vec<u32> = re
        .captures_iter(lines.next().unwrap())
        .map(|cap| cap.get(1).unwrap().as_str().parse().unwrap())
        .collect::<Vec<_>>();
    let records: Vec<u32> = re
        .captures_iter(lines.next().unwrap())
        .map(|cap| cap.get(1).unwrap().as_str().parse().unwrap())
        .collect::<Vec<_>>();

    let product: u32 = zip(times, records)
        .map(|(time, record)| {
            (0..time)
                .map(|hold| hold * (time - hold))
                .filter(|distance| *distance > record)
                .count() as u32
        })
        .product();

    Some(product)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let re = Regex::new(r"((\d+\s+)+\d+)").unwrap();
    let time: u64 = re
        .captures(lines.next().unwrap())
        .map(|cap| {
            cap.get(1)
                .unwrap()
                .as_str()
                .replace(' ', "")
                .parse()
                .unwrap()
        })
        .unwrap();
    let record: u64 = re
        .captures(lines.next().unwrap())
        .map(|cap| {
            cap.get(1)
                .unwrap()
                .as_str()
                .replace(' ', "")
                .parse()
                .unwrap()
        })
        .unwrap();

    let wins = (0..time)
        .map(|hold| (hold * (time - hold)))
        .filter(|distance| *distance > record)
        .count();

    Some(wins as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
