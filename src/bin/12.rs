use advent_of_code::template::aoc_cli::check;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (chars, spec_str) = line.split_once(' ').unwrap();
        let mut springs: Vec<_> = chars.chars().map(|c| Spring::try_from(c).unwrap()).collect();
        let spec: Vec<_> = spec_str.split(',').map(|d| d.parse::<u8>().unwrap()).collect();

        let line_count = get_count(&mut springs, &spec, 0);
        println!("{}", line_count);
        sum += line_count;
        
    }

    Some(sum)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Spring {
    Working,
    Broken,
    Unknown
}

impl TryFrom<char> for Spring {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '?' => Ok(Spring::Unknown),
            '#' =>  Ok(Spring::Broken),
            '.' => Ok(Spring::Working),
            _ => Err("Unknown spring")
        }
    }
}

fn get_count(springs: &mut Vec<Spring>, spec: &Vec<u8>, start: usize) -> u32 {
    if start == springs.len() || !springs[start..springs.len()].contains(&Spring::Unknown) {
        return if check_spec(springs, spec) {
            1
        } else {
            0
        }
    };

    let mut sum = 0;
    for i in start..springs.len() {
        if matches!(springs[i], Spring::Unknown) {
            springs[i] = Spring::Working;
            sum += get_count(springs, spec, i + 1);

            springs[i] = Spring::Broken;
            sum += get_count(springs, spec, i + 1);

            springs[i] = Spring::Unknown;

            break;
        }
    }

    sum
}



fn check_spec(springs: &Vec<Spring>, spec: &Vec<u8>) -> bool {
    let mut spec_idx = 0;
    let mut in_chunk = false;
    let mut chunk_len = 0;
    for spring in springs {
        match spring {
            Spring::Broken => {
                if in_chunk {
                    chunk_len += 1;
                } else {
                    if spec.len() <= spec_idx {
                        return false;
                    }
                    in_chunk = true;
                    chunk_len = 1;
                }
            },
            Spring::Working => {
                if in_chunk {
                    if spec[spec_idx] != chunk_len {
                        return false;
                    }
                    in_chunk = false;
                    chunk_len = 0;
                    spec_idx += 1;
                }
            },
            // only works for known
            Spring::Unknown => return false,
        }
    }
    if in_chunk {
        if spec[spec_idx] != chunk_len {
            return false;
        }
        spec_idx += 1;
    } 

    let response = if spec_idx != spec.len() {
        false
    } else {
        true
    };

    // let input = &advent_of_code::template::read_file_part("examples", DAY, 2);

    // for line in input.lines() {
    //     let allowed_spring: Vec<_> = line.chars().map(|c| Spring::try_from(c).unwrap()).collect();
    //     if springs.eq(&allowed_spring) {
    //         if response {
    //             println!("Allowed response: {:?}", springs);
    //             return true;
    //         } else {
    //             println!("Reponse was not allowed but should have been response: {:?}", springs);
    //             return false;
    //         }
    //     }
    // }
    // if response {
    //     println!("Reponse was allowed but should not have been response: {:?}", springs);
    // }
    response
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (chars, spec_str) = line.split_once(' ').unwrap();
        let joined = vec![chars; 5].join("?");
        let mut springs: Vec<_> = joined.chars().map(|c| Spring::try_from(c).unwrap()).collect();
        let joined_spec = vec![spec_str; 5].join(",");
        let spec: Vec<_> = joined_spec.split(',').map(|d| d.parse::<u8>().unwrap()).collect();

        let line_count = get_count(&mut springs, &spec, 0);
        println!("{}", line_count);
        sum += line_count;
        
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_match() {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let spec = vec![3,2,1];
        for line in input.lines() {
            let springs: Vec<_> = line.chars().map(|c| Spring::try_from(c).unwrap()).collect();
            println!("{:?}", springs);
            assert_eq!(true, check_spec(&springs, &spec))

        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
