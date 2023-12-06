advent_of_code::solution!(5);

use std::cmp;

use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|seed| seed.parse().unwrap())
        .collect();

    let re = Regex::new(r"map:\n([\d\s]+)[\n\w]").unwrap();
    let mut current_inputs = seeds.clone();
    re.captures_iter(input).for_each(|capture| {
        let mut next_inputs: Vec<i64> = vec![-1; current_inputs.len()];
        for line in capture.get(1).unwrap().as_str().lines() {
            if line.trim().is_empty() {
                continue;
            }

            let (new_range_start, old_range_start, range_length): (i64, i64, i64) = line
                .split(' ')
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .unwrap();

            for i in 0..current_inputs.len() {
                let input = current_inputs[i];
                if old_range_start <= input && input < old_range_start + range_length {
                    next_inputs[i] = new_range_start + input - old_range_start;
                }
            }
        }
        for (i, input) in next_inputs.iter().enumerate() {
            if *input != -1 {
                current_inputs[i] = next_inputs[i]
            }
        }
    });

    Some(*current_inputs.iter().min().unwrap() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let pairs_re = Regex::new(r"((\d+) (\d+))").unwrap();
    let mut seed_ranges: Vec<(i64, i64)> = Vec::new();
    let mut sum: i64 = 0;
    pairs_re
        .captures_iter(lines.next().unwrap().split_once(": ").unwrap().1)
        .for_each(|capture| {
            let (start_str, range_str) = (capture.get(2), capture.get(3));
            let start: i64 = start_str.unwrap().as_str().parse().unwrap();
            let range: i64 = range_str.unwrap().as_str().parse().unwrap();
            seed_ranges.push((start, range));
            sum += range;
        });
    println!("Sum: {}", sum);
    // this should stay constant throughout
    let total_seeds: i64 = seed_ranges.iter().map(|x| x.1).sum();

    let re = Regex::new(r"map:\n([\d\s]+)[\n\w]").unwrap();
    let mut current_ranges = seed_ranges.clone();
    re.captures_iter(input).for_each(|capture| {
        let mut mapped_ranges: Vec<(i64, i64)> = Vec::new();
        for line in capture.get(1).unwrap().as_str().lines() {
            if line.trim().is_empty() {
                continue;
            }

            let mut unmapped_ranges: Vec<(i64, i64)> = Vec::new();
            let (new_range_start, old_range_start, range_length): (i64, i64, i64) = line
                .split(' ')
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .unwrap();
            // when a find a range with some amount of overlap, we need to remove *only the overlapping* part
            // from the current ranges and add to the new ranges
            // At the very end, we add all ranges back in
            for (start, range) in current_ranges.iter() {
                // construct a new range
                // overlap is max of the low bounds and min of the upper bounds
                let overlap_start = cmp::max(*start, old_range_start);
                let overlap_end = cmp::min(start + range, old_range_start + range_length);

                // there are 3 cases
                //  1. no overlap at all - do nothing
                //  2. mapping completely covers the range - remove entire range from current_ranges and add mapped to next_ranges
                //  3. mapping partially covers the range - remove only overlapping porition from current ranges, add the mapped
                //      porition to next_ranges, and add the non-overlapping porition back to current_ranges
                // no overlap on this new range
                if overlap_start > overlap_end {
                    // just save the original range
                    unmapped_ranges.push((*start, *range));
                    continue;
                }
                // there is some overlap, add the mapped option
                let mapped_start = new_range_start + overlap_start - old_range_start;
                let mapped_range = overlap_end - overlap_start;
                mapped_ranges.push((mapped_start, mapped_range));

                // there is incomplete coverage on the left side of the range
                if overlap_start > *start {
                    unmapped_ranges.push((*start, overlap_start - *start))
                }
                // there is incomplete coverage on the right side of the range
                if overlap_end < start + range {
                    unmapped_ranges.push((overlap_end, *start + range - overlap_end))
                }
            }
            // Now the ranges under consideration only contain those that have not been mapped
            current_ranges = unmapped_ranges;
        }
        // Add all ranges that were unmapped back in
        mapped_ranges.extend(current_ranges.iter());

        // We should be maintaining a constant number of seeds (total of ranges)
        assert_eq!(mapped_ranges.iter().map(|x| x.1).sum::<i64>(), total_seeds);
        current_ranges = mapped_ranges;
    });

    Some(current_ranges.iter().map(|x| x.0).min().unwrap() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
