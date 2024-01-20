use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let first_bytes: HashSet<&u8> = HashSet::from_iter(first.as_bytes().iter());
        let second_bytes: HashSet<&u8> = HashSet::from_iter(second.as_bytes().iter());

        let common = first_bytes.intersection(&second_bytes);
        let val = **common.last().unwrap() as u32;

        sum += if val > 90 { val - 96 } else { val - 38 }
    }
    Some(sum)
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
