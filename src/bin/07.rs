use std::{cmp::Ordering, iter::zip};

use itertools::Itertools;

advent_of_code::solution!(7);

const ORDER_1: &[char] = &[
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const ORDER_2: &[char] = &[
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Eq, PartialEq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    hand: Vec<char>,
    part: Part,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // changes between 1 and 2
        let order = match self.part {
            Part::One => ORDER_1,
            Part::Two => ORDER_2,
        };
        if self.hand_type == other.hand_type {
            for (c, other_c) in zip(&self.hand, &other.hand) {
                if c != other_c {
                    let self_idx = order.iter().position(|&r| r == *c).unwrap();
                    let other_idx = order.iter().position(|&r| r == *other_c).unwrap();
                    return Ord::cmp(&self_idx, &other_idx);
                }
            }
            Ordering::Equal
        } else {
            Ord::cmp(&self.hand_type, &other.hand_type)
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum Part {
    One,
    Two,
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoOfAKind,
    OnePair,
    HighCard,
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let (hand_str, bet_str) = line.split_once(' ').unwrap();
            let hand: Vec<char> = hand_str.chars().collect();
            let bet: u32 = bet_str.parse().unwrap();

            (
                Hand {
                    hand_type: get_type_1(&hand),
                    hand,
                    part: Part::One,
                },
                bet,
            )
        })
        .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
        .collect();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, (_, bet))| (i as u32 + 1) * bet)
            .sum(),
    )
}

fn get_type_1(chars: &[char]) -> HandType {
    let unique = chars.iter().counts();
    match unique.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if *unique.values().max().unwrap() == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if *unique.values().max().unwrap() == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoOfAKind
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("frick"),
    }
}

fn get_type_2(chars: &[char]) -> HandType {
    let unique = chars.iter().counts();
    if !chars.contains(&'J') {
        return get_type_1(chars);
    }
    let max = unique
        .iter()
        .filter(|(k, _)| ***k != 'J')
        .max_by(|a, b| Ord::cmp(a.1, b.1));

    if max.is_none() {
        return HandType::FiveOfAKind;
    }
    let (max_char, _) = max.unwrap();
    let new_chars: Vec<char> = chars
        .iter()
        .map(|c| if *c == 'J' { **max_char } else { *c })
        .collect();

    get_type_1(&new_chars)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let (hand_str, bet_str) = line.split_once(' ').unwrap();
            let hand: Vec<char> = hand_str.chars().collect();
            let bet: u32 = bet_str.parse().unwrap();

            (
                Hand {
                    hand_type: get_type_2(&hand),
                    hand,
                    part: Part::Two,
                },
                bet,
            )
        })
        .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
        .collect();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, (_, bet))| (i as u32 + 1) * bet)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
