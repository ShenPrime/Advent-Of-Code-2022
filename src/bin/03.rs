use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let first_bytes: HashSet<&u8> = HashSet::from_iter(first.as_bytes().iter());
        let second_bytes: HashSet<&u8> = HashSet::from_iter(second.as_bytes().iter());
        // first_bytes.iter().fold(sum, |acc, e| {
        //     if second_bytes.contains(e) && e > &90 {
        //         acc + (e - 96)
        //     } else if second_bytes.contains(e) {
        //         acc + (e - 64)
        //     } else {
        //         acc
        //     }
        // });

        // first_bytes.into_iter().for_each(|b| {
        //     if second_bytes.contains(b) && b > &90 {
        //         sum += *b as u32 - 96
        //     } else if second_bytes.contains(b) {
        //         sum += *b as u32 - 38
        //     }
        // })

        for &val in first_bytes {
            if second_bytes.contains(&val) && val > 90 {
                sum += val as u32 - 96
            } else if second_bytes.contains(&val) {
                sum += val as u32 - 38
            }
        }
    }
    Some(sum.into())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
