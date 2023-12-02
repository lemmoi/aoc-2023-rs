advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mut sum = 0;
    for line in lines {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                last_digit = Some(digit);
            };
        }
        sum += (10 * first_digit?) + last_digit?;
    }
    Some(sum)
}

const WORD_DIGITS: &'static [&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mut sum = 0;
    let mut word: Vec<char> = Vec::with_capacity(100);
    for line in lines {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        'chars: for c in line.chars() {
            match c.to_digit(10) {
                Some(digit) => {
                    first_digit = Some(digit);
                    break;
                }
                None => word.push(c),
            };
            let potential_string = word.iter().rev().take(5).rev().collect::<String>();
            for (idx, digit_word) in WORD_DIGITS.iter().enumerate() {
                if potential_string.ends_with(digit_word) {
                    first_digit = Some(idx as u32 + 1);
                    break 'chars;
                }
            }
        }
        word.clear();

        'chars_rev: for c in line.chars().rev() {
            match c.to_digit(10) {
                Some(digit) => {
                    last_digit = Some(digit);
                    break;
                }
                None => word.push(c),
            };
            let potential_string = word.iter().rev().take(5).collect::<String>();
            for (idx, digit_word) in WORD_DIGITS.iter().enumerate() {
                if potential_string.starts_with(digit_word) {
                    last_digit = Some(idx as u32 + 1);
                    break 'chars_rev;
                }
            }
        }
        sum += (10 * first_digit?) + last_digit?;
        word.clear();
    }
    Some(sum)
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
        let result = part_two("5four6fb4four3twocn");
        assert_eq!(result, Some(52));
    }
}
