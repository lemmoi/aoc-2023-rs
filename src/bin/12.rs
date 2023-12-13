use std::{cmp, collections::HashMap};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (chars, spec_str) = line.split_once(' ').unwrap();
        let springs: Vec<_> = chars.chars().map(|c| Spring::try_from(c).unwrap()).collect();
        let spec: Vec<_> = spec_str.split(',').map(|d| d.parse::<u8>().unwrap()).collect();

        let line_count = get_count(&springs, &spec, &mut HashMap::new());
        sum += line_count;
        
    }

    Some(sum as u32)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
enum Spring {
    Working = b'.',
    Broken = b'#',
    Unknown = b'?'
}

impl Into<char> for Spring {
    fn into(self) -> char {
        self as u8 as char
    }
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


#[derive(Eq, PartialEq, Hash)]
struct CacheKey {
    springs: String,
    spec: String
}

impl CacheKey {
    fn new(springs: &[Spring], spec: &[u8]) -> Self { Self {
        springs: springs.iter().map(|c| -> char {<Spring as Into<char>>::into(*c)}).collect(),
        spec: spec.iter().map(|c| *c as char).collect() 
    } }
}

fn memoize(springs: &[Spring], spec: &[u8], memo: &mut HashMap<CacheKey, u64>) -> u64 {
    let key = CacheKey::new(springs, spec);

    if let Some(result) = memo.get(&key) {
        *result
    } else {
        let computed = get_count(springs, spec, memo);
        memo.insert(key, computed);
        computed
    }
}


fn get_count(springs: &[Spring], spec: &[u8], memo: &mut HashMap<CacheKey, u64>) -> u64 {
    // If there are no springs left, then its valid iff there are no more chunks
    if springs.len() == 0 {
        if spec.len() == 0 {
            return 1;
        }
        return 0;
    }

    // If there are no chunks of broken springs left, it's valid iff there are no broken springs
    if spec.len() == 0 {
        if springs.iter().any(|s| matches!(s, Spring::Broken)) {
            return 0;
        }
        return 1;
    }

    // Check if there is enough room for the required remaining broken springs
    // + the springs that must separate them
    if springs.len() < spec.iter().sum::<u8>() as usize + spec.len() - 1 {
        return 0;
    }

    return match springs[0] {
        Spring::Working => {
            // this spring is working, just evaluate on the rest
            memoize(&springs[1..], spec, memo)
        },
        Spring::Broken => {
            // this spring is broken, check if it can fit the next chunk in the best case
            let chunk_size = spec[0] as usize;
            // if there are any working springs, this can't work
            if springs[..chunk_size].iter().any(|s| matches!(s, Spring::Working)) {
                return 0;
            }

            // if the spring at the end of the chunk does not work, this can't work
            if matches!(springs.get(chunk_size).unwrap_or(&Spring::Working), Spring::Broken) {
                return 0;
            }

            memoize(&springs[cmp::min(springs.len(), chunk_size + 1)..], &spec[1..], memo)
        },
        Spring::Unknown => {
            memoize(&[&[Spring::Broken][..], &springs[1..]].concat(), spec, memo) + 
            memoize(&[&[Spring::Working][..], &springs[1..]].concat(), spec, memo)
        },
    };
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let (chars, spec_str) = line.split_once(' ').unwrap();
        let joined = vec![chars; 5].join("?");
        let springs: Vec<_> = joined.chars().map(|c| Spring::try_from(c).unwrap()).collect();
        let joined_spec = vec![spec_str; 5].join(",");
        let spec: Vec<_> = joined_spec.split(',').map(|d| d.parse::<u8>().unwrap()).collect();

        let line_count = get_count(&springs, &spec, &mut HashMap::new());
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
