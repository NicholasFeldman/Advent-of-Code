fn to_priority(c: char) -> usize {
    match c {
        'a'..='z' => (c as usize) - ('a' as usize) + 1,
        'A'..='Z' => (c as usize) - ('A' as usize) + 27,
        _ => unimplemented!(),
    }
}

fn split_compartments(string: &str) -> (&str, &str) {
    string.split_at(string.len() / 2)
}

fn find_in_both(left: &str, right: &str) -> char {
    left.chars().filter(|c| right.contains(*c)).collect::<Vec<char>>()[0]
}

fn find_in_three(one: &str, two: &str, three: &str) -> char {
    one.chars().filter(|c| two.contains(*c) && three.contains(*c)).collect::<Vec<char>>()[0]
}


pub fn part_one(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let (left, right) = split_compartments(line);
        total += to_priority(find_in_both(left, right)) as u32;
    }

    total
}

pub fn part_two(input: &str) -> u32 {
    let mut total: u32 = 0;

    let collected = input.lines().collect::<Vec<_>>();
    let chunks = collected.chunks_exact(3);

    for chunk in chunks {
        let bag1 = chunk[0];
        let bag2 = chunk[1];
        let bag3 = chunk[2];
        let badge = find_in_three(bag1, bag2, bag3);
        total += to_priority(badge) as u32
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", "day03.txt");
        assert_eq!(part_one(&input), 157);
    }

#[test]
fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", "day03.txt");
        assert_eq!(part_two(&input), 70);
    }
}