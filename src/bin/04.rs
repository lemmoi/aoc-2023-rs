use winnow::{
    ascii::{digit1, space1},
    combinator::separated,
    stream::AsChar,
    token::{take_till, take_while},
    PResult, Parser,
};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input
        .lines()
        .map(|mut line| {
            let _: PResult<&str> = take_till(1.., ':').parse_next(&mut line);
            take_til_digit(&mut line).unwrap();
            let mut winning_nums = get_nums_vec(&mut line).unwrap();
            take_til_digit(&mut line).unwrap();
            let my_nums = get_nums_vec(&mut line).unwrap();
            // At the size of these inputs, iterating over both vecs
            // is faster than building and intersecting std HashSet
            winning_nums.retain(|item| my_nums.contains(item));
            let num_won = winning_nums.len();
            if num_won != 0 {
                (1 << (num_won - 1)) as u32
            } else {
                0
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut wins_per_game: Vec<u8> = Vec::new();
    for mut line in input.lines() {
        let _: PResult<&str> = take_till(1.., ':').parse_next(&mut line);
        take_til_digit(&mut line).unwrap();
        let mut winning_nums = get_nums_vec(&mut line).unwrap();
        take_til_digit(&mut line).unwrap();
        let my_nums = get_nums_vec(&mut line).unwrap();

        // At the size of these inputs, iterating over both vecs
        // is faster than building and intersecting std HashSet
        winning_nums.retain(|item| my_nums.contains(item));
        wins_per_game.push(winning_nums.len() as u8);
    }

    let mut num_games_by_idx: Vec<u32> = vec![0; wins_per_game.len()];
    for i in 0..wins_per_game.len() {
        num_games_by_idx[i] += 1;
        let wins_this_game = wins_per_game[i];
        for j in 0..(wins_this_game as usize) {
            let index = j + i + 1;
            if index < wins_per_game.len() {
                num_games_by_idx[index] += num_games_by_idx[i];
            }
        }
    }
    Some(num_games_by_idx.iter().sum())
}

fn take_til_digit<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., |c| !AsChar::is_dec_digit(c)).parse_next(input)
}

fn get_nums_vec(input: &mut &str) -> PResult<Vec<u8>> {
    separated(1.., digit1.parse_to::<u8>(), space1).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
