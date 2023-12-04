advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let (winning_nums_str, my_nums_str) =
                line.split_once(':').unwrap().1.split_once('|').unwrap();
            let mut winning_nums = get_nums_vec(winning_nums_str);
            let my_nums = get_nums_vec(my_nums_str);
            // At the size of these inputs, iterating over both vecs
            // is faster than building and intersecting std HashSet
            winning_nums.retain(|item| my_nums.contains(item));
            let num_won = winning_nums.len();
            (1 << (num_won - 1)) * (num_won != 0) as u32
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut wins_per_game: Vec<u8> = Vec::new();
    for line in input.lines() {
        let (winning_nums_str, my_nums_str) =
            line.split_once(':').unwrap().1.split_once('|').unwrap();
        let mut winning_nums = get_nums_vec(winning_nums_str);
        let my_nums = get_nums_vec(my_nums_str);
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

fn get_nums_vec(input: &str) -> Vec<u8> {
    input
        .split(' ')
        .filter_map(|num| num.trim().parse::<u8>().ok())
        .collect()
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
