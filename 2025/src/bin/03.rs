advent_of_code::solution!(3);

#[derive(Debug, Clone)]
struct Bank {
    batteries: Vec<u32>,
}

impl From<&str> for Bank {
    fn from(value: &str) -> Self {
        let batteries = value
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        Bank { batteries }
    }
}

impl Bank {
    fn max_joltage(&self, max_switches: usize) -> u64 {
        let mut max_joltage = 0;
        let mut left_idx = 0;
        for switch in 1..=max_switches {
            let (max_idx, max_value) = self
                .batteries
                .iter()
                .skip(left_idx)
                .take(self.batteries.len() - (max_switches - switch) - left_idx)
                .enumerate()
                .rev()
                // max by key will not prioritize earlier indices on ties, so we rev
                .max_by_key(|&(_, x)| x)
                .unwrap();
            left_idx += max_idx + 1;
            max_joltage = max_joltage * 10 + (*max_value as u64);
        }
        max_joltage
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let total: u64 = input
        .lines()
        .map(|line| {
            let bank = Bank::from(line);
            bank.max_joltage(2)
        })
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total: u64 = input
        .lines()
        .map(|line| {
            let bank = Bank::from(line);
            bank.max_joltage(12)
        })
        .sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
