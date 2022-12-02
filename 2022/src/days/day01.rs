fn top_three(input: &str) -> Vec<u32> {
    let mut current = 0;
    let mut all = Vec::new();

    for line in input.split("\n") {
        if line.is_empty() {
            all.push(current);
            current = 0;
            continue;
        }

        current += line.parse::<u32>().unwrap();
    }

    all.sort_by(|a, b| b.cmp(a));
    all[0..3].to_vec()
}

pub fn part_one(input: &str) -> u32 {
    top_three(input)[0]
}

pub fn part_two(input: &str) -> u32 {
    top_three(input).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", "day01.txt");
        assert_eq!(part_one(&input), 24000);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", "day01.txt");
        assert_eq!(part_two(&input), 45000);
    }
}