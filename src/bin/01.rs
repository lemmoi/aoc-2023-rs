advent_of_code::solution!(1);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mut sum = 0;
    for line in lines {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        for c in line.chars() {
            match c.to_digit(10) {
                Some(digit) => {
                    if first_digit.is_none() {
                        first_digit = Some(digit);
                    }
                    last_digit = Some(digit);
                }
                None => (),
            };
        }
        sum += (10 * first_digit?) + last_digit?;
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_forward =
        Regex::new(r"((one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|[1-9])")
            .unwrap();
    let re_reverse =
        Regex::new(r"((eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)|[1-9])")
            .unwrap();
    let lines = input.split('\n');
    let mut sum = 0;
    for line in lines {
        let digits_foward: Vec<u32> = re_forward
            .find_iter(line)
            .map(|digit| as_numeric(digit.as_str()))
            .collect();
        let digits_reverse: Vec<u32> = re_reverse
            .find_iter(&line.chars().rev().collect::<String>())
            .map(|digit| as_numeric(digit.as_str()))
            .collect();
        sum += (10 * digits_foward.first()?) + digits_reverse.first()?;
    }
    Some(sum)
}

fn as_numeric(input: &str) -> u32 {
    match input.len() {
        1 => input.chars().nth(0).unwrap().to_digit(10).unwrap(),
        _ => match input {
            "one" | "eno" => 1,
            "two" | "owt" => 2,
            "three" | "eerht" => 3,
            "four" | "ruof" => 4,
            "five" | "evif" => 5,
            "six" | "xis" => 6,
            "seven" | "neves" => 7,
            "eight" | "thgie" => 8,
            "nine" | "enin" => 9,
            _ => panic!("Unknown digit"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two("6oneighthlf");
        assert_eq!(result, Some(68));
    }
}
