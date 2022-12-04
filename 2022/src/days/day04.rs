use std::ops::RangeInclusive;

type AssignmentPair = (RangeInclusive<u8>, RangeInclusive<u8>);

fn one_contains_other(pair: AssignmentPair) -> bool {
    // One contains two
    (pair.0.contains(&pair.1.start()) && pair.0.contains(&pair.1.end()))
    // Two contains one
    || (pair.1.contains(&pair.0.start()) && pair.1.contains(&pair.0.end()))
}

fn has_any_overlap(pair: AssignmentPair) -> bool {
    // One overlaps two
    (pair.0.contains(&pair.1.start()) || pair.0.contains(&pair.1.end()))
    // Two overlaps one
    || (pair.1.contains(&pair.0.start()) || pair.1.contains(&pair.0.end()))
}

fn to_assignment_pair(string: &str) -> AssignmentPair {
    let re = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let cap = re.captures(string).unwrap();

    return (
        RangeInclusive::new(
            cap.get(1).unwrap().as_str().parse::<u8>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<u8>().unwrap(),
        ),
        RangeInclusive::new(
            cap.get(3).unwrap().as_str().parse::<u8>().unwrap(),
            cap.get(4).unwrap().as_str().parse::<u8>().unwrap(),
        ),
    )

}

pub fn part_one(input: &str) -> u32 {
    input.lines().fold::<u32, _>(0, |acc, line| {
        if one_contains_other(to_assignment_pair(line)) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn part_two(input: &str) -> u32 {
    input.lines().fold::<u32, _>(0, |acc, line| {
        if has_any_overlap(to_assignment_pair(line)) {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", "day04.txt");
        assert_eq!(part_one(&input), 2);
    }

#[test]
fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", "day04.txt");
        assert_eq!(part_two(&input), 4);
    }
}