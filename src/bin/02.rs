use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let combinations = HashMap::from([
        ("A X", 4),
        ("B Y", 5),
        ("C Z", 6),
        ("A Y", 8),
        ("B Z", 9),
        ("C X", 7),
        ("A Z", 3),
        ("B X", 1),
        ("C Y", 2),
    ]);

    let mut sum = 0;

    for line in input.lines() {
        if let Some(val) = combinations.get(&line) {
            sum += val;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let combinations: HashMap<&str, HashMap<&str, u32>> = HashMap::from([
        ("Y", HashMap::from([("A X", 4), ("B Y", 5), ("C Z", 6)])),
        ("X", HashMap::from([("A Z", 3), ("B X", 1), ("C Y", 2)])),
        ("Z", HashMap::from([("A Y", 8), ("B Z", 9), ("C X", 7)])),
    ]);

    for line in input.lines() {
        let chars: Vec<&str> = line.split_whitespace().collect();
        //["A", "X"]

        if let Some(inner_map) = combinations.get(&chars[1]) {
            for (key, &value) in inner_map {
                if key.starts_with(&chars[0]) {
                    sum += value;
                    break;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
